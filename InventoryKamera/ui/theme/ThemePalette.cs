using System.Drawing;

namespace InventoryKamera.ui.theme
{
    public class ThemePalette
    {
        public bool IsDark { get; set; }
        public string Name { get; set; }

        // Window & Container Backgrounds
        public Color Background { get; set; }
        public Color CardBackground { get; set; }
        public Color CardBorder { get; set; }
        public Color Border => CardBorder;

        // Surfaces (Inputs, Lists, PictureBoxes)
        public Color SurfaceBackground { get; set; }
        public Color SurfaceBorder { get; set; }

        // Text
        public Color TextPrimary { get; set; }
        public Color TextSecondary { get; set; }
        public Color TextDisabled { get; set; }

        // Buttons
        public Color ButtonBack { get; set; }
        public Color ButtonHover { get; set; }
        public Color ButtonPressed { get; set; }
        public Color ButtonText { get; set; }
        public Color ButtonBorder { get; set; }

        // Links & Accents
        public Color Link { get; set; }
        public Color Accent { get; set; }

        // Logs
        public Color LogBackground { get; set; }
        public Color LogForeground { get; set; }
        public Color LogInfo { get; set; }
        public Color LogWarning { get; set; }
        public Color LogError { get; set; }

        // Menu
        public Color MenuBackground { get; set; }
        public Color MenuBorder { get; set; }
        public Color MenuItemHover { get; set; }
        public Color MenuText { get; set; }

        public static ThemePalette CreateDark()
        {
            return new ThemePalette
            {
                IsDark = true,
                Name = "Dark",
                Background = Color.FromArgb(28, 28, 30),
                CardBackground = Color.FromArgb(36, 36, 38),
                CardBorder = Color.FromArgb(58, 58, 62),
                SurfaceBackground = Color.FromArgb(44, 44, 46),
                SurfaceBorder = Color.FromArgb(70, 70, 75),
                TextPrimary = Color.FromArgb(242, 242, 247),
                TextSecondary = Color.FromArgb(174, 174, 178),
                TextDisabled = Color.FromArgb(99, 99, 102),
                ButtonBack = Color.FromArgb(50, 50, 54),
                ButtonHover = Color.FromArgb(64, 64, 70),
                ButtonPressed = Color.FromArgb(40, 40, 44),
                ButtonText = Color.FromArgb(242, 242, 247),
                ButtonBorder = Color.FromArgb(76, 76, 82),
                Link = Color.FromArgb(100, 160, 255),
                Accent = Color.FromArgb(10, 132, 255),
                LogBackground = Color.FromArgb(22, 22, 24),
                LogForeground = Color.FromArgb(235, 235, 240),
                LogInfo = Color.FromArgb(115, 215, 140),
                LogWarning = Color.FromArgb(255, 205, 80),
                LogError = Color.FromArgb(255, 105, 100),
                MenuBackground = Color.FromArgb(32, 32, 34),
                MenuBorder = Color.FromArgb(60, 60, 64),
                MenuItemHover = Color.FromArgb(55, 55, 60),
                MenuText = Color.FromArgb(242, 242, 247)
            };
        }

        public static ThemePalette CreateLight()
        {
            return new ThemePalette
            {
                IsDark = false,
                Name = "Light",
                Background = Color.FromArgb(242, 242, 247),
                CardBackground = Color.FromArgb(255, 255, 255),
                CardBorder = Color.FromArgb(216, 216, 220),
                SurfaceBackground = Color.FromArgb(255, 255, 255),
                SurfaceBorder = Color.FromArgb(200, 200, 205),
                TextPrimary = Color.FromArgb(28, 28, 30),
                TextSecondary = Color.FromArgb(108, 108, 112),
                TextDisabled = Color.FromArgb(174, 174, 178),
                ButtonBack = Color.FromArgb(242, 242, 247),
                ButtonHover = Color.FromArgb(229, 229, 234),
                ButtonPressed = Color.FromArgb(218, 218, 222),
                ButtonText = Color.FromArgb(28, 28, 30),
                ButtonBorder = Color.FromArgb(199, 199, 204),
                Link = Color.FromArgb(0, 122, 255),
                Accent = Color.FromArgb(0, 122, 255),
                LogBackground = Color.FromArgb(255, 255, 255),
                LogForeground = Color.FromArgb(28, 28, 30),
                LogInfo = Color.FromArgb(20, 130, 60),
                LogWarning = Color.FromArgb(180, 100, 0),
                LogError = Color.FromArgb(210, 45, 40),
                MenuBackground = Color.FromArgb(248, 248, 250),
                MenuBorder = Color.FromArgb(216, 216, 220),
                MenuItemHover = Color.FromArgb(230, 230, 235),
                MenuText = Color.FromArgb(28, 28, 30)
            };
        }
    }
}
