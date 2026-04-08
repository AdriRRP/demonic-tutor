export function supportsQrSignalImport(): boolean {
  return typeof BarcodeDetector !== "undefined" && typeof createImageBitmap === "function";
}

export function supportsLiveQrSignalImport(): boolean {
  return (
    typeof BarcodeDetector !== "undefined" &&
    typeof navigator !== "undefined" &&
    "mediaDevices" in navigator &&
    typeof navigator.mediaDevices.getUserMedia === "function"
  );
}

export async function readQrSignalFromSource(source: ImageBitmapSource): Promise<string> {
  if (typeof BarcodeDetector === "undefined") {
    throw new Error("This browser cannot detect QR codes yet.");
  }

  const detector = new BarcodeDetector({ formats: ["qr_code"] });
  const candidates = await detector.detect(source);
  const payload = candidates
    .map((candidate) => candidate.rawValue?.trim() ?? "")
    .find((candidate) => candidate.length > 0);

  if (!payload) {
    throw new Error("No QR signaling payload was found.");
  }

  return payload;
}

export async function readQrSignalFromFile(file: File): Promise<string> {
  if (!supportsQrSignalImport()) {
    throw new Error("This browser cannot detect QR codes from images yet.");
  }

  const bitmap = await createImageBitmap(file);

  try {
    return await readQrSignalFromSource(bitmap);
  } finally {
    bitmap.close();
  }
}
