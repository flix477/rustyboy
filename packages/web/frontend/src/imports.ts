export async function imports() {
  const rustyboy = await import('rustyboy-web');
  const Gameboy = await import('./gameboy');
  return {rustyboy, Gameboy};
}