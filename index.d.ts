/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare class DirectoryWatcher {
  static new(callback: (err: null | Error, result: string) => void | Promise<void>): DirectoryWatcher
  watch(path: string): void
  unwatch(path: string): void
  getWatchedPaths(): Array<string>
}
