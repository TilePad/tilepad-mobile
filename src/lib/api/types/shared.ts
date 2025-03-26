export type Uuid = string;

export type Option<T> = T | null;

export type DeepPartial<T> = T extends object
  ? {
      [P in keyof T]?: DeepPartial<T[P]>;
    }
  : T;

export type TilePosition = {
  row: number;
  column: number;
};
