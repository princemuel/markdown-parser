type MapKeys<M extends Map<unknown, unknown>> = Prettify<
  Array<Parameters<M["get"]>[0]>
>;
type Prettify<T> = { [K in keyof T]: T[K] } & NonNullable<unknown>;
