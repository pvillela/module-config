/*
  Based on https://stackoverflow.com/a/48342359/445619
 */

export class ConfigurationError extends Error {
  constructor(msg: string) {
    // 'Error' breaks prototype chain here
    super(msg);

    // restore prototype chain
    const actualProto = new.target.prototype;

    if (Object.setPrototypeOf) { Object.setPrototypeOf(this, actualProto); }
    else {
      // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-explicit-any
      (this as any).__proto__ = actualProto;
    }
  }
}
