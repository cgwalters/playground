test

#define _USE_GNU 1
#include <fcntl.h>
#include <stdio.h>
#include <dirent.h>
#include <sys/stat.h>
#include <stdarg.h>
#include <errno.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <stdlib.h>

static void fatal (const char *message, ...) __attribute__ ((noreturn)) __attribute__ ((format (printf, 1, 2)));
static void fatal_errno (const char *message) __attribute__ ((noreturn));

static void
fatal (const char *fmt,
       ...)
{
  va_list args;
  
  va_start (args, fmt);

  vfprintf (stderr, fmt, args);
  putc ('\n', stderr);
  
  va_end (args);
  exit (1);
}

static void
fatal_errno (const char *message)
{
  perror (message);
  exit (1);
}

int
main (int argc, char **argv)
{
  int fd = openat (AT_FDCWD, argv[1], O_RDONLY | O_NONBLOCK | O_DIRECTORY | O_CLOEXEC | O_NOCTTY);
  DIR *d = NULL;
  struct dirent *de;

  if (fd == -1)
    fatal_errno ("openat");

  d = fdopendir (fd);
  if (d == NULL)
    fatal_errno ("fdopendir");

  do
    {
      errno = 0;
      de = readdir (d);
      if (de == NULL)
	{
	  if (errno != 0)
	    fatal_errno ("readdir");
	  
	  break;
	}
      else
	{
	  if (strcmp ((de)->d_name, ".") == 0 ||
              strcmp ((de)->d_name, "..") == 0)
	    continue;

	  printf ("%s\n", de->d_name);
	}
    } while (de);

  exit (0);
}
