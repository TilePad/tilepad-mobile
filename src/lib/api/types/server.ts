export interface ServerConnectionInfo {
  interfaces: ServerInterface[];
  port: number;
}

export interface ServerInterface {
  name: string;
  addr: string;
}

export interface EncodedInterfaces {
  addr: string[];
  port: number;
}
