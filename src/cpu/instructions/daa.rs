// DAA seems to be subtly different in every member of the 8080 family (8080, 8085, Z80, and GB CPU). This is what I can suss out from the source code of several emulators:
//
// On the Z80:
//
// If C is set OR a > 0x99, add or subtract 0x60 depending on N, and set C
// If H is set OR (a & 0xf) > 9, add or subtract 6 depending on N
//
// H is set if bit 4 of A changed, otherwise cleared.
// C is set as described above; note that DAA never clears the C flag if it was already set
// (that would break multi-byte BCD arithmetic).
// N is preserved, and the rest of the flags are set the usual way (P/V is parity, not overflow)
// On the GB:
//
// DAA after an add (N flag clear) works the same way as on the Z80.
// DAA after a subtract (N flag set) only tests the C and H flags, and not the previous value of a.
// H is always cleared (for both add and subtract).
// N is preserved, Z is set the usual way, and the rest of the Z80 flags don't exist.
// I think the Z80 and GB effectively behave the same when DAA is used "legally" (after an add/subtract between two legal BCD values); the differences only matter if you use DAA after subtracting/comparing illegal BCD values.