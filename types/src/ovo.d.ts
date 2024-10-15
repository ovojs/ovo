declare module "ovo" {

  interface Reader {
    read(b: Uint8Array): Promise<number>;
  }

  interface Writer {
    write(b: Uint8Array): Promise<number>;
  }

  interface Closer {
    close(): Promise<void>;
  }

  type ReadWriter = Reader & Writer;
  type ReadWriteCloser = Reader & Writer & Closer;

  interface File extends ReadWriteCloser {
    fd: number;
    stat(): Promise<FileInfo>;
    sync(): Promise<void>;
  }

  interface FileInfo {
    size: number;
    isDirectory: boolean;
    createdAt: Date;
    modifiedAt?: Date;
  }

  interface OpenOptions {
    read: boolean;
    write: boolean;
    append: boolean;
    create: boolean;
    truncate: boolean;
    mode: number;
  }

  /**
   * Open a file. The file does not have to be created already.
   * 
   * @example
   * ```ts
   * const file = await OvO.open("hello.txt", { write: true });
   * await file.write(new Uint8Array([0x68, 0x65, 0x6C, 0x6C, 0x6F]));
   * await file.close();
   * ```
   */
  function open(path: string, options?: OpenOptions): Promise<File>;
}
