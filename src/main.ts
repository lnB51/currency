import './styles/app.css'
import App from './app.svelte'
import { invoke } from '@tauri-apps/api'
import { appDataDir } from '@tauri-apps/api/path'
import { PhysicalPosition, PhysicalSize, appWindow, currentMonitor } from '@tauri-apps/api/window';
import { platform } from "@tauri-apps/api/os";

const monitor = currentMonitor();
const targetOS = await platform();

// Center window on monitor and set default width & height
monitor.then((monitor) => {
  if (monitor && monitor.size) {
    const screenWidth = monitor.size.width;
    const screenHeight = monitor.size.height;
    if (screenWidth !== undefined && screenHeight !== undefined) {
      const window_size = new PhysicalSize(800, 600);
      const windowX = (screenWidth - window_size.width) / 2;
      const windowY = (screenHeight - window_size.height) / 2;
      const window_position = new PhysicalPosition(
        windowX,
        targetOS === 'darwin' ? windowY / 2 : windowY
      );
      appWindow.setSize(window_size);
      appWindow.setPosition(window_position);
    } else {
      throw new Error('Failed to retrieve screen size.');
    }
  } else {
    throw new Error('Failed to retrieve monitor information.');
  }
});

// On startup make request to function for API processing
invoke("fetch_and_save_currency_rates", { data_dir: await appDataDir(), manual: "false" })

const app = new App({
  target: document.getElementById('app'),
})

export default app

// document.addEventListener('contextmenu', event => event.preventDefault())

// Titlebar buttons functionality
document
  .getElementById('titlebar-minimize')
  .addEventListener('click', () => appWindow.minimize())
document
  .getElementById('titlebar-maximize')
  .addEventListener('click', () => appWindow.toggleMaximize())
document
  .getElementById('titlebar-close')
  .addEventListener('click', () => appWindow.close())

// Window starts in hidden mode to prevent "white screen" loading, here we finally render it
appWindow.show();
