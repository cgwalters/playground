#include <string.h>
#include <gio/gio.h>

static void
consume_error (GError *error)
{
  g_printerr ("%s\n", error->message);
  g_error_free (error);
}

static int opt_idle_timeout = 10;
static int opt_save_timeout = 2;

typedef struct {
  GMainContext *mainctx;
  gboolean running;

  GFile *counterf;

  guint counter;
  GSource *idle_save_source;
  GSource *idle_exit_source;
} App;

static GDBusNodeInfo *introspection_data = NULL;

/* Introspection data for the service we are exporting */
static const gchar introspection_xml[] =
  "<node>"
  "  <interface name='org.verbum.Counter'>"
  "    <method name='Inc'>"
  "    </method>"
  "    <method name='Get'>"
  "      <arg direction='out' type='u'/>"
  "    </method>"
  "  </interface>"
  "</node>";

static gboolean
idle_exit (App *self)
{
  g_printerr ("Exiting on idle\n");
  self->running = FALSE;
  g_main_context_wakeup (self->mainctx);
}

static void
bump_idle_timer (App *self)
{
  if (self->idle_exit_source)
    g_source_destroy (self->idle_exit_source);

  g_printerr ("Reset idle timer (%u seconds)\n", opt_idle_timeout);
  self->idle_exit_source = g_timeout_source_new_seconds (opt_idle_timeout);
  g_source_set_callback (self->idle_exit_source, (GSourceFunc)idle_exit, self, NULL);
  g_source_attach (self->idle_exit_source, self->mainctx);
}

static gboolean
idle_save (App *self)
{
  GError *local_error = NULL;
  char *counter_str = NULL;

  self->idle_save_source = NULL;

  counter_str = g_strdup_printf ("%u\n", self->counter);

  g_printerr ("Performing idle content save...");
  if (!g_file_replace_contents (self->counterf, counter_str, strlen (counter_str),
				NULL, FALSE, 0,
				NULL, NULL, &local_error))
    goto out;
  g_printerr ("Done\n");

 out:
  if (local_error)
    consume_error (local_error);

  g_free (counter_str);

  return FALSE;
}

static void
handle_method_call (GDBusConnection       *connection,
                    const gchar           *sender,
                    const gchar           *object_path,
                    const gchar           *interface_name,
                    const gchar           *method_name,
                    GVariant              *parameters,
                    GDBusMethodInvocation *invocation,
                    gpointer               user_data)
{
  App *self = user_data;

  if (g_strcmp0 (method_name, "Get") == 0)
    {
      g_dbus_method_invocation_return_value (invocation, g_variant_new ("(u)", self->counter));
    }
  else if (g_strcmp0 (method_name, "Inc") == 0)
    {
      self->counter++;
      if (self->idle_save_source == NULL)
	{
	  self->idle_save_source = g_timeout_source_new_seconds (opt_save_timeout);
	  g_source_set_callback (self->idle_save_source, (GSourceFunc)idle_save, self, NULL);
	  g_source_attach (self->idle_save_source, self->mainctx);
	}
      bump_idle_timer (self);
      g_dbus_method_invocation_return_value (invocation, NULL);
    }
}

/* for now */
static const GDBusInterfaceVTable counter_interface_vtable =
{
  handle_method_call,
  NULL,
  NULL 
};

static void
on_bus_acquired (GDBusConnection *connection,
                 const gchar     *name,
                 gpointer         user_data)
{
  guint id = g_dbus_connection_register_object (connection,
						"/org/verbum/counter",
						introspection_data->interfaces[0],
						&counter_interface_vtable,
						user_data,
						NULL,
						NULL);
  g_assert_cmpuint (id, >, 0);
}

static void
on_name_acquired (GDBusConnection *connection,
                  const gchar     *name,
                  gpointer         user_data)
{
}

static void
on_name_lost (GDBusConnection *connection,
              const gchar     *name,
              gpointer         user_data)
{
}

int
main (int argc, char **argv)
{
  App app = { 0, };
  GError *local_error = NULL;
  guint owner_id;
  GOptionContext *option_context;
  GOptionEntry option_entries[] =
    {
      { "idle-timeout", 'i', 0, G_OPTION_ARG_INT, &opt_idle_timeout, "Idle timeout in seconds", "SECONDS" },
      { "save-timeout", 's', 0, G_OPTION_ARG_INT, &opt_save_timeout, "Save timeout in seconds", "SECONDS" },
      { NULL}
    };

  introspection_data = g_dbus_node_info_new_for_xml (introspection_xml, NULL);
  g_assert (introspection_data != NULL);

  option_context = g_option_context_new ("GDBus exit on idle");
  g_option_context_add_main_entries (option_context, option_entries, NULL);

  if (!g_option_context_parse (option_context, &argc, &argv, &local_error))
    goto out;

  app.counterf = g_file_new_for_path ("counter");

  owner_id = g_bus_own_name (G_BUS_TYPE_SESSION,
                             "org.verbum.TestExitOnIdle",
                             0,
                             on_bus_acquired,
                             on_name_acquired,
                             on_name_lost,
                             &app,
                             NULL);
  g_assert_cmpuint (owner_id, >, 0);

  { char *contents;
    gsize len;

    if (!g_file_load_contents (app.counterf, NULL, &contents, &len,
			       NULL, &local_error))
      goto out;

    app.counter = g_ascii_strtoull (contents, NULL, 10);
  }

  app.running = TRUE;
  bump_idle_timer (&app);

  while (app.running)
    g_main_context_iteration (app.mainctx, TRUE);

  if (app.idle_save_source)
    (void) idle_save (&app);

 out:
  return 0;
}
