/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface JsBm42Options {
  alpha?: number
  cacheDir?: string
  showDownloadProgress?: boolean
}
export interface JsSparseEmbedding {
  indices: Array<number>
  values: Array<number>
}
export type JsBM42 = BM42
export declare class BM42 {
  constructor(opts?: JsBm42Options | undefined | null)
  embed(texts: Array<string>): Array<JsSparseEmbedding>
  queryEmbed(texts: Array<string>): Array<JsSparseEmbedding>
}
