export type AppCfgInfo = {
  x: string;
  y: number;
};

export function getAppConfiguration(): AppCfgInfo {
  return {
    x: "xxx",
    y: 42,
  };
}
