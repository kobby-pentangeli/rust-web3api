/* eslint-disable @typescript-eslint/naming-convention */
/* tslint:disable */
/**
 * This file was automatically generated by scripts/manifest/index-ts.mustache.
 * DO NOT MODIFY IT BY HAND. Instead, modify scripts/manifest/index-ts.mustache,
 * and run node ./scripts/manifest/generateFormatTypes.js to regenerate this file.
 */

import {
  BuildManifest as BuildManifest0_0_1_prealpha_2
} from "./0.0.1-prealpha.2";

export {
  BuildManifest0_0_1_prealpha_2,
};

export enum BuildManifestFormats {
  "0.0.1-prealpha.2" = "0.0.1-prealpha.2",
}

export type AnyBuildManifest =
  | BuildManifest0_0_1_prealpha_2

export type BuildManifest = BuildManifest0_0_1_prealpha_2;

export const latestBuildManifestFormat = BuildManifestFormats["0.0.1-prealpha.2"]

export { migrateBuildManifest } from "./migrate";