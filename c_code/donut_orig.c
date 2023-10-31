k;
double sin ();
double cos ();
main ()
{
    const R1 = 1;
    const R2 = 2;
    const K2 = 5;
    const Z_DIST = 0;
    const SCREEN_WIDTH = 80;
    const SCREEN_HEIGHT = 40;
    const K1 = SCREEN_WIDTH*(K2 - Z_DIST)*3/(8*(R1+R2));
    const BUFFER_SIZE = SCREEN_WIDTH * SCREEN_HEIGHT;
    float A = 0;
    float B = 0;
    float i;
    float j;
    float z_loc[BUFFER_SIZE];
    char b[BUFFER_SIZE];
    printf ("\x1b[2J");
    for (;;)
    {
        memset (b, 32, BUFFER_SIZE);
        memset (z_loc, 0, BUFFER_SIZE * 4);
        float e = sin (A);
        float g = cos (A);
        float m = cos (B);
        float n = sin (B);
        for (j = 0; 6.28 > j; j += 0.07)
        {
            float d = cos (j);
            float f = sin (j);
            
            for (i = 0; 6.28 > i; i += 0.02)
            {
                float c = sin (i);
                float l = cos (i);
                float h = R1*d + R2;
                float z = c * h * e + f * g + K2;
                float ooz = 1 / z;
                float t = c * h * g - f * e;
                int x = SCREEN_WIDTH / 2 + K1 * ooz * (l * h * m - t * n);
                int y = SCREEN_HEIGHT / 2 + 2 + K1/2 * ooz * (l * h * n + t * m);
                int o = x + SCREEN_WIDTH * y;
                int N = 8 * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n);
                if (SCREEN_HEIGHT > y && y > 0 && x > 0 && SCREEN_WIDTH > x && ooz > z_loc[o])
                {
                    z_loc[o] = ooz;;;
                    b[o] = ".,-~:;=!*#$@"[N > 0 ? N : 0];
                }
            }
        }
        printf ("\x1b[H");
        for (k = 0; BUFFER_SIZE > k; k++)
        {
            putchar (k % SCREEN_WIDTH ? b[k] : 10);
        }
        A += 0.04;
        B += 0.02;
    }
}
