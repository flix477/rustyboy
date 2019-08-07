export async function imports() {
  const rustyboy = await import('rustyboy-web');
  const Emulator = await import('./emulator');
  return {rustyboy, Emulator};
}