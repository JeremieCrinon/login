// This function gets an env variable and throws if it's not set
export function env(key: string): string {
  const value = process.env[key];
  if (!value) throw new Error(`Environment variable ${key} is not set but is required.`)
  return value;
}
