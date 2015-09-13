  $ export PROGRAM="${TESTDIR}/../../../target/release/tr"

  $ $PROGRAM
  SET1 is required
   (glob)
  Usage: tr - [OPTION]... SET1 [SET2]
   (glob)
  Options:
      -c, --complement    use the complement of SET1
      -C                  same as -c
      -d, --delete        delete characters in SET1, do not translate
      -s, --squeeze       replace each input sequence of a repeated character
                          that is listed in SET1 with a single occurrence of
                          that character
      -t, --truncate-set1  (glob)
                          first truncate SET1 to length of SET2
      -h, --help          display this help and exit

  $ $PROGRAM set1
  SET1 and SET2 are required for translation.
   (glob)
  Usage: tr - [OPTION]... SET1 [SET2]
   (glob)
  Options:
      -c, --complement    use the complement of SET1
      -C                  same as -c
      -d, --delete        delete characters in SET1, do not translate
      -s, --squeeze       replace each input sequence of a repeated character
                          that is listed in SET1 with a single occurrence of
                          that character
      -t, --truncate-set1  (glob)
                          first truncate SET1 to length of SET2
      -h, --help          display this help and exit

  $ $PROGRAM -t set1
  -t may only be used when translating.
   (glob)
  Usage: tr - [OPTION]... SET1 [SET2]
   (glob)
  Options:
      -c, --complement    use the complement of SET1
      -C                  same as -c
      -d, --delete        delete characters in SET1, do not translate
      -s, --squeeze       replace each input sequence of a repeated character
                          that is listed in SET1 with a single occurrence of
                          that character
      -t, --truncate-set1  (glob)
                          first truncate SET1 to length of SET2
      -h, --help          display this help and exit

  $ $PROGRAM -ds set1
  Two strings must be given when both deleting and squeezing repeats.
   (glob)
  Usage: tr - [OPTION]... SET1 [SET2]
   (glob)
  Options:
      -c, --complement    use the complement of SET1
      -C                  same as -c
      -d, --delete        delete characters in SET1, do not translate
      -s, --squeeze       replace each input sequence of a repeated character
                          that is listed in SET1 with a single occurrence of
                          that character
      -t, --truncate-set1  (glob)
                          first truncate SET1 to length of SET2
      -h, --help          display this help and exit

  $ $PROGRAM -z set1
  Unrecognized option: 'z'.
   (glob)
  Usage: tr - [OPTION]... SET1 [SET2]
   (glob)
  Options:
      -c, --complement    use the complement of SET1
      -C                  same as -c
      -d, --delete        delete characters in SET1, do not translate
      -s, --squeeze       replace each input sequence of a repeated character
                          that is listed in SET1 with a single occurrence of
                          that character
      -t, --truncate-set1  (glob)
                          first truncate SET1 to length of SET2
      -h, --help          display this help and exit
