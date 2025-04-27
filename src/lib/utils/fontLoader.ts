const loadedFonts = new Set<string>();
const failedFonts = new Set<string>();
const loadingFonts = new Set<string>();

/**
 * Requests that a new font be loaded from the tilepad server
 *
 * @param serverURL URL of the server
 * @param fontName Name of the font
 * @param bold Whether to get the bold variant
 * @param italic Whether to get the italic variant
 */
export function loadFont(
  serverURL: string,
  fontName: string,
  bold: boolean,
  italic: boolean,
) {
  const fontKey = `${fontName}-${bold}-${italic}`;

  // Don't load if its already loaded, is currently loading, or has already failed loading
  if (
    loadedFonts.has(fontKey) ||
    loadingFonts.has(fontKey) ||
    failedFonts.has(fontKey)
  )
    return;

  // Mark the font as currently loading
  loadingFonts.add(fontKey);

  // Create API params
  const params = new URLSearchParams();
  params.append("bold", `${bold}`);
  params.append("italic", `${italic}`);

  // Create a URL source pointing to the fonts API
  const source = `url(${serverURL}/fonts/${encodeURIComponent(fontName)}?${params.toString()})`;

  const font = new FontFace(fontName, source, {
    style: italic ? "italic" : "normal",
    weight: bold ? "700" : "400",
  });

  font
    .load()
    .then((font) => {
      // Add the font to the loaded fonts
      document.fonts.add(font);
      loadedFonts.add(fontKey);
    })
    .catch((cause) => {
      console.error("failed to load font", cause);
      // Add the font to the failed fonts list
      failedFonts.add(fontKey);
    })
    .finally(() => {
      // Remove the font from the loading list
      loadingFonts.delete(fontKey);
    });
}
