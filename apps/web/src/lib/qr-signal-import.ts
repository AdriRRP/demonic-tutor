export function supportsQrSignalImport(): boolean {
  return typeof BarcodeDetector !== "undefined" && typeof createImageBitmap === "function";
}

export async function readQrSignalFromFile(file: File): Promise<string> {
  if (!supportsQrSignalImport()) {
    throw new Error("This browser cannot detect QR codes from images yet.");
  }

  const detector = new BarcodeDetector({ formats: ["qr_code"] });
  const bitmap = await createImageBitmap(file);

  try {
    const candidates = await detector.detect(bitmap);
    const payload = candidates
      .map((candidate) => candidate.rawValue?.trim() ?? "")
      .find((candidate) => candidate.length > 0);

    if (!payload) {
      throw new Error("No QR signaling payload was found in the selected image.");
    }

    return payload;
  } finally {
    bitmap.close();
  }
}
