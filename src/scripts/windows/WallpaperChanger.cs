using System;
using System.Runtime.InteropServices;

public class Wallpaper
{
    [DllImport("user32.dll", CharSet = CharSet.Auto)]
    private static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);

    public static void SetWallpaper(string path)
    {
        SystemParametersInfo(20, 0, path, 0x01 | 0x02);
    }

    public static int Main(string[] args)
    {
        try
        {
            string wallpaperPath = args[0];
            SetWallpaper(wallpaperPath);
            return 0;
        }
        catch (Exception ex)
        {
            Console.WriteLine("Error setting wallpaper: " + ex.Message);
            return 1;
        }
    }
}
