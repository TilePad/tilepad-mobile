import { Context } from "runed";

interface ServerContext {
  serverURL: string;
  deviceId: string;
}

export const serverContext = new Context<ServerContext>("ServerContext");
