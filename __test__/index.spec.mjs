import test from "ava";

import { BM42 } from "../index.js";

test("embed()", (t) => {
  let bm42 = new BM42({
    alpha: 0.5,
    showDownloadProgress: false,
  });

  let texts = [
    "It's a truth universally acknowledged that a zombie in possession of brains must be in want of more brains.",
    "We're not in Infinity; we're in the suburbs.",
    "I was a thousand times more evil than thou!",
    "History is merely a list of surprises... It can only prepare us to be surprised yet again.",
  ];

  let results = bm42.embed(texts);
  t.is(results.length, 4);
});

test("query_embed()", (t) => {
  let bm42 = new BM42();
  let results = bm42.queryEmbed([
    "One day he will be",
    "the king of the world",
    "and I will rule",
    "with an iron fist",
    "and a heart of gold",
  ]);
  t.is(results.length, 5);
});