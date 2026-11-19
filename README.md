# Cosmic Launcher (BETA)

Minecraft Launcher + FPS-Boost Client (Tauri v2 + React + Three.js).

## Đã có (Windows / macOS / Linux)
- Chạy Minecraft Vanilla, Fabric, Quilt; tự tải game + Java (Adoptium)
- FPS Boost: cờ JVM tối ưu, tự cài Sodium/Lithium/FerriteCore/ImmediatelyFast/Entity Culling, preset đồ họa
- Kho Mod Modrinth 1-click (tự cài dependency), bật/tắt mod
- AI Auto-Fixer (Anthropic / OpenAI / Gemini / Ollama): đọc crash-report, tắt mod lỗi, cài mod thiếu, tăng RAM, chạy lại
- Xem skin / áo choàng 3D, lưu theo từng tài khoản
- Tài khoản Khách / Offline, thông báo từ trang web

## Chưa có
Forge / NeoForge, đăng nhập Microsoft, CurseForge, áp skin vào game (offline), chạy game trên Android / iOS.

## Trang web thông báo
1. Sửa `WEB_URL` trong `src/config.ts` thành `https://<tên-github>.github.io/<tên-repo>/`.
2. Trên GitHub: Settings -> Pages -> Source = GitHub Actions.
3. Sửa `site/news.json` rồi push, workflow "Deploy web thông báo" sẽ đăng lên.

## Chạy thử giao diện
    npm install
    npm run dev

## Build
Tab Actions -> Build Windows / macOS / Android / iOS -> Run workflow -> tải ở Artifacts.
