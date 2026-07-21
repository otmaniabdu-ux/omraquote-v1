declare module 'vue' {
  interface ComponentCustomProperties {
    $t: (key: string, ...params: unknown[]) => string;
    $router: { push(path: string): void };
    $route: any;
    setTimeout: (fn: () => void, ms: number) => number;
  }
}

export {};
