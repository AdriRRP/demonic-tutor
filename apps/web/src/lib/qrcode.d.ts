declare module "qrcode" {
  export interface ToStringOptions {
    color?: {
      dark?: string;
      light?: string;
    };
    errorCorrectionLevel?: "L" | "M" | "Q" | "H";
    margin?: number;
    type?: "svg" | "terminal" | "utf8";
    width?: number;
  }

  export function toString(text: string, options?: ToStringOptions): Promise<string>;

  const QRCode: {
    toString(text: string, options?: ToStringOptions): Promise<string>;
  };

  export default QRCode;
}
