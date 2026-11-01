pub fn check_latest_version(cur: &str) -> Option<(String, String)> {
    let script = r#"
URL="https://github.com/hassanve/gridcraft/releases/"
DL_DIR="$HOME/Downloads"
mkdir -p "$DL_DIR"

START_MARKER=$(mktemp)
notify-send "GridCraft Updater | 1405/08/10" "صفحه Releases باز شد. هر نسخه‌ای را که می‌خواهید دانلود کنید تا پس از اتمام دانلود، به صورت خودکار نصب شود..." 2>/dev/null || true
xdg-open "$URL" >/dev/null 2>&1 &

for i in $(seq 1 900); do
    sleep 2
    FOUND=$(find "$DL_DIR" -maxdepth 1 \( -name "gridcraft*.AppImage" -o -name "gridcraft*.deb" -o -name "gridcraft*.rpm" -o -name "gridcraft*.tar.zst" -o -name "gridcraft*.flatpak" -o -name "gridcraft*.snap" \) -newer "$START_MARKER" 2>/dev/null | head -n 1)
    if [ -n "$FOUND" ] && [ ! -f "${FOUND}.part" ] && [ ! -f "${FOUND}.crdownload" ]; then
        S1=$(stat -c%s "$FOUND" 2>/dev/null || echo 0)
        sleep 2
        S2=$(stat -c%s "$FOUND" 2>/dev/null || echo 0)
        if [ "$S1" -gt 100000 ] && [ "$S1" = "$S2" ]; then
            rm -f "$START_MARKER"
            notify-send "GridCraft Updater" "دانلود کامل شد! در حال نصب خودکار فایل انتخابی شما..." 2>/dev/null || true
            case "$FOUND" in
                *.AppImage)
                    chmod +x "$FOUND"
                    if [ -n "$APPIMAGE" ]; then
                        mv "$FOUND" "$APPIMAGE"
                        notify-send "GridCraft Updater" "نسخه جدید AppImage جایگزین و اجرا شد!" 2>/dev/null || true
                        exec "$APPIMAGE"
                    else
                        notify-send "GridCraft Updater" "نسخه AppImage آماده و اجرا شد!" 2>/dev/null || true
                        exec "$FOUND"
                    fi
                    ;;
                *.deb)
                    pkexec dpkg -i "$FOUND"
                    ;;
                *.rpm)
                    pkexec rpm -Uvh --force "$FOUND"
                    ;;
                *.tar.zst)
                    pkexec pacman -U --noconfirm "$FOUND"
                    ;;
                *.flatpak)
                    flatpak install --user -y "$FOUND" || flatpak-spawn --host pkexec flatpak install -y "$FOUND" || pkexec flatpak install -y "$FOUND"
                    ;;
                *.snap)
                    pkexec snap install --dangerous "$FOUND"
                    ;;
            esac
            notify-send "GridCraft Updater" "فرآیند نصب خودکار با موفقیت به پایان رسید!" 2>/dev/null || true
            exit 0
        fi
    fi
done
rm -f "$START_MARKER"

"#;
    let _ = std::process::Command::new("sh").arg("-c").arg(script).spawn();
    Some((String::from(cur), String::from("https://github.com/hassanve/gridcraft/releases/")))
}
