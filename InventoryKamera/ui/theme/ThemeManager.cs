using System;
using System.Drawing;
using System.Runtime.InteropServices;
using System.Windows.Forms;
using Microsoft.Win32;

namespace InventoryKamera.ui.theme
{
    public enum AppTheme
    {
        System = 0,
        Dark = 1,
        Light = 2
    }

    public static class ThemeManager
    {
        [DllImport("dwmapi.dll", PreserveSig = true)]
        private static extern int DwmSetWindowAttribute(IntPtr hwnd, int attr, ref int attrValue, int attrSize);

        private const int DWMWA_USE_IMMERSIVE_DARK_MODE_BEFORE_20H1 = 19;
        private const int DWMWA_USE_IMMERSIVE_DARK_MODE = 20;

        public static event Action ThemeChanged;

        private static AppTheme currentMode = AppTheme.System;

        public static AppTheme CurrentMode
        {
            get => currentMode;
            set
            {
                if (currentMode != value)
                {
                    currentMode = value;
                    ThemeChanged?.Invoke();
                }
            }
        }

        public static ThemePalette CurrentPalette
        {
            get
            {
                switch (currentMode)
                {
                    case AppTheme.Dark:
                        return ThemePalette.CreateDark();
                    case AppTheme.Light:
                        return ThemePalette.CreateLight();
                    case AppTheme.System:
                    default:
                        return IsSystemDark() ? ThemePalette.CreateDark() : ThemePalette.CreateLight();
                }
            }
        }

        public static bool IsSystemDark()
        {
            try
            {
                using (var key = Registry.CurrentUser.OpenSubKey(@"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"))
                {
                    if (key != null)
                    {
                        var value = key.GetValue("AppsUseLightTheme");
                        if (value is int intVal)
                        {
                            return intVal == 0;
                        }
                    }
                }
            }
            catch
            {
                // Registry access failed or running on older OS, fallback to system metrics below
            }

            // Fallback / Windhawk check: if system control text has bright luminance or system window is dark,
            // the user is running an active dark theme/skin.
            return SystemColors.ControlText.GetBrightness() > 0.5f || SystemColors.Window.GetBrightness() < 0.4f;
        }

        public static void UpdateDwmTitleBar(Form form)
        {
            if (form == null || form.IsDisposed || !form.IsHandleCreated) return;

            try
            {
                int useDark = CurrentPalette.IsDark ? 1 : 0;
                DwmSetWindowAttribute(form.Handle, DWMWA_USE_IMMERSIVE_DARK_MODE, ref useDark, sizeof(int));
                DwmSetWindowAttribute(form.Handle, DWMWA_USE_IMMERSIVE_DARK_MODE_BEFORE_20H1, ref useDark, sizeof(int));
            }
            catch
            {
                // Silently ignore on older Windows versions
            }
        }

        public static void Apply(Form form)
        {
            if (form == null || form.IsDisposed) return;

            var palette = CurrentPalette;
            form.BackColor = palette.Background;
            form.ForeColor = palette.TextPrimary;

            UpdateDwmTitleBar(form);

            foreach (Control control in form.Controls)
            {
                ApplyToControl(control, palette);
            }
        }

        public static void ApplyToControl(Control control, ThemePalette palette)
        {
            if (control == null || control.IsDisposed) return;

            if (control is MenuStrip menuStrip)
            {
                menuStrip.BackColor = palette.MenuBackground;
                menuStrip.ForeColor = palette.MenuText;
                menuStrip.Renderer = new ToolStripProfessionalRenderer(new ThemeMenuColorTable(palette));
                StyleToolStripItems(menuStrip.Items, palette);
                return;
            }

            if (control is GroupBox groupBox)
            {
                groupBox.BackColor = palette.CardBackground;
                groupBox.ForeColor = palette.TextPrimary;
            }
            else if (control is Button button)
            {
                button.UseVisualStyleBackColor = false;
                button.FlatStyle = FlatStyle.Flat;
                button.FlatAppearance.BorderSize = 1;
                button.FlatAppearance.BorderColor = palette.ButtonBorder;
                button.BackColor = palette.ButtonBack;
                button.ForeColor = palette.ButtonText;
                button.FlatAppearance.MouseOverBackColor = palette.ButtonHover;
                button.FlatAppearance.MouseDownBackColor = palette.ButtonPressed;
            }
            else if (control is TextBox textBox)
            {
                textBox.BackColor = palette.SurfaceBackground;
                textBox.ForeColor = palette.TextPrimary;
                textBox.BorderStyle = BorderStyle.FixedSingle;
            }
            else if (control is NumericUpDown num)
            {
                num.BackColor = palette.SurfaceBackground;
                num.ForeColor = palette.TextPrimary;
                num.BorderStyle = BorderStyle.FixedSingle;
            }
            else if (control is ComboBox comboBox)
            {
                comboBox.BackColor = palette.SurfaceBackground;
                comboBox.ForeColor = palette.TextPrimary;
                comboBox.FlatStyle = FlatStyle.Flat;
            }
            else if (control is ListBox listBox)
            {
                listBox.BackColor = palette.SurfaceBackground;
                listBox.ForeColor = palette.TextPrimary;
                listBox.BorderStyle = BorderStyle.FixedSingle;
            }
            else if (control is CheckBox checkBox)
            {
                checkBox.ForeColor = palette.TextPrimary;
                checkBox.BackColor = Color.Transparent;
            }
            else if (control is RadioButton radioButton)
            {
                radioButton.ForeColor = palette.TextPrimary;
                radioButton.BackColor = Color.Transparent;
            }
            else if (control is LinkLabel linkLabel)
            {
                linkLabel.LinkColor = palette.Link;
                linkLabel.ActiveLinkColor = palette.Link;
                linkLabel.VisitedLinkColor = palette.Link;
                linkLabel.BackColor = Color.Transparent;
            }
            else if (control is Label label)
            {
                label.ForeColor = palette.TextPrimary;
                label.BackColor = Color.Transparent;
            }
            else if (control is PictureBox pictureBox)
            {
                pictureBox.BackColor = palette.SurfaceBackground;
                pictureBox.BorderStyle = BorderStyle.FixedSingle;
            }
            else if (control is TrackBar trackBar)
            {
                trackBar.BackColor = palette.CardBackground;
            }
            else if (control is SplitContainer split)
            {
                split.Panel1.BackColor = palette.Background;
                split.Panel2.BackColor = palette.Background;
                split.BackColor = palette.Border;
            }
            else if (control is Panel || control is TableLayoutPanel || control is FlowLayoutPanel)
            {
                // Keep transparent or match parent
                if (control.Parent is GroupBox)
                    control.BackColor = palette.CardBackground;
                else
                    control.BackColor = palette.Background;
            }

            foreach (Control child in control.Controls)
            {
                ApplyToControl(child, palette);
            }
        }

        private static void StyleToolStripItems(ToolStripItemCollection items, ThemePalette palette)
        {
            foreach (ToolStripItem item in items)
            {
                item.ForeColor = palette.MenuText;
                item.BackColor = palette.MenuBackground;
                if (item is ToolStripDropDownItem dropDownItem && dropDownItem.HasDropDownItems)
                {
                    StyleToolStripItems(dropDownItem.DropDownItems, palette);
                }
            }
        }

        private class ThemeMenuColorTable : ProfessionalColorTable
        {
            private readonly ThemePalette palette;

            public ThemeMenuColorTable(ThemePalette palette)
            {
                this.palette = palette;
                UseSystemColors = false;
            }

            public override Color MenuBorder => palette.MenuBorder;
            public override Color MenuItemBorder => palette.MenuBorder;
            public override Color ToolStripDropDownBackground => palette.MenuBackground;
            public override Color ImageMarginGradientBegin => palette.MenuBackground;
            public override Color ImageMarginGradientMiddle => palette.MenuBackground;
            public override Color ImageMarginGradientEnd => palette.MenuBackground;
            public override Color MenuItemSelected => palette.MenuItemHover;
            public override Color MenuItemSelectedGradientBegin => palette.MenuItemHover;
            public override Color MenuItemSelectedGradientEnd => palette.MenuItemHover;
            public override Color MenuItemPressedGradientBegin => palette.MenuItemHover;
            public override Color MenuItemPressedGradientMiddle => palette.MenuItemHover;
            public override Color MenuItemPressedGradientEnd => palette.MenuItemHover;
            public override Color MenuStripGradientBegin => palette.MenuBackground;
            public override Color MenuStripGradientEnd => palette.MenuBackground;
            public override Color SeparatorDark => palette.MenuBorder;
            public override Color SeparatorLight => palette.MenuBorder;
        }
    }
}
