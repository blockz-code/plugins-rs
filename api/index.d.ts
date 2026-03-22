export {};



declare global {
  interface Window {
    storePlugins: object;
    registerPlugin<Type> (name: string, object: Type): void;
    loadPlugin<Type> (name: string): Type;
    countPlugins: () => number;
  }
}