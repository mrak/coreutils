Deletion

  $ printf "delete part of me" | ${TESTDIR}/../../target/tr -d eao
  dlt prt f m (no-eol)

Deletion with squeezing

  $ printf "delete     part    of      meeeee" | ${TESTDIR}/../../target/tr -ds eao ' e'
  dlt prt f m (no-eol)

Squeezing without deleting

  $ printf "delete     part    of      mme" | ${TESTDIR}/../../target/tr -s ' m'
  delete part of me (no-eol)

Squeezing with multiple characters

  $ printf "delete     part    of      meeeeeeeeeeeeeeeeeeee" | ${TESTDIR}/../../target/tr -s 'e '
  delete part of me (no-eol)

Complementary deletion

  $ printf "delete     part    of      meeeeeeeeeeeeeeeeeeee" | ${TESTDIR}/../../target/tr -cd 'e '
  eee               eeeeeeeeeeeeeeeeeeee (no-eol)
  $ printf "delete     part    of      meeeeeeeeeeeeeeeeeeee" | ${TESTDIR}/../../target/tr -Cd 'e '
  eee               eeeeeeeeeeeeeeeeeeee (no-eol)

Complementary squeezing

  $ printf "delete     part    of      meeeeeeeeeeeeeeeeeeee" | ${TESTDIR}/../../target/tr -cs me
  delete part of meeeeeeeeeeeeeeeeeeee (no-eol)

Translation with extending SET2

  $ printf "delete     part    of      meeeeeeeeeeeeeeeeeeee" | ${TESTDIR}/../../target/tr meda op
  pplptp     pprt    of      opppppppppppppppppppp (no-eol)

Translation with truncation of SET1

  $ printf "delete     part    of      meeeeeeeeeeeeeeeeeeee" | ${TESTDIR}/../../target/tr -t meda op
  dplptp     part    of      opppppppppppppppppppp (no-eol)
