#include <stdio.h>

int main()
{
  for (int i = 1; i <= 10; i++)
  {
    printf("%d", i);
  }

  int i = 1;
  while (1)
  {
    printf("%d", i);
    i++;

    if (i > 10)
    {
      break;
    }
  }
}