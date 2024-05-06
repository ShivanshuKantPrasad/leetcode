#include <stdio.h>
#include <string.h>
// Absolute function
int abs(int x) { return (x < 0) ? -x : x; }

// Min function
int min(int x, int y) { return (x < y) ? x : y; }

int min_steps_loop(char *ring, char *key) {
  int nk = strlen(key);
  int nr = strlen(ring);

  int distance[100][100];

  int indices[26][100] = {[0 ... 25][0 ... 99] = -1};
  for (int i = 0; i < 26; i++) {
    int index = 0;
    int ch = 'a' + i;
    for (int j = 0; j < nr; j++) {
      if (ring[j] == ch) {
        indices[i][index] = j;
        index++;
      }
    }
  }

  for (int key_index = nk - 1; key_index > 0; key_index--) {
    int x = 0;
    while (indices[key[key_index - 1] - 'a'][x] != -1) {
      int ring_index = indices[key[key_index - 1] - 'a'][x];
      int res = 1000000;

      int y = 0;
      while (indices[key[key_index] - 'a'][y] != -1) {
        int char_index = indices[key[key_index] - 'a'][y];
        int dist = abs(ring_index - char_index);
        dist = min(dist, nr - dist);

        int foo =
            1 + dist +
            ((key_index == nk - 1) ? 0 : distance[char_index][key_index + 1]);

        res = min(res, foo);
        y++;
      }
      distance[ring_index][key_index] = res;

      x++;
    }
  }

  int key_index = 0;
  int ring_index = 0;
  int res = 1000000;

  int y = 0;
  while (indices[key[key_index] - 'a'][y] != -1) {
    int char_index = indices[key[key_index] - 'a'][y];
    int dist = abs(ring_index - char_index);
    dist = min(dist, nr - dist);

    int foo = 1 + dist +
              ((key_index == nk - 1) ? 0 : distance[char_index][key_index + 1]);

    res = min(res, foo);
    y++;
  }
  distance[0][0] = res;
  return distance[0][0];
}

int main() {
  volatile long long a = 0;
  for (int i = 0; i < 100; i++) {
    a += min_steps_loop("godding", "gd");
    a += min_steps_loop("godding", "godding");
    a += min_steps_loop("abcde", "ade");
    a += min_steps_loop("y", "y");
    a += min_steps_loop(
        "dccjgocjnscdalkwrlzmsrhnprvvsvqjtqnzirtaasdeldiiokttozjf"

        "kwjghhaibtzkoepdvfkhxgmxwtwsrgiryzqljpsntjei",
        "lsdsvjtj");
    a += min_steps_loop(
        "zmgddxyivijgullayjbxlmcbirsgpaeqemefpshtehczznvqjaetauso"

        "agpkqjtewszqihnzevhbcgpmmtdbyesgxjulyzshnksc",
        "yxebsleaklhkjlx");
    a += min_steps_loop(
        "gyzhbpwtmixjdmozuctjctsvcdxtpbtaftuqqirabgmnibnanegqxivx"

        "ljloodnmmylisompbtccwksvxcdxoixgjibhkmsujdvx",
        "tihsvtysmktbjtjkcvirxbixbx");
    a += min_steps_loop(
        "cimrihxbkmotdfudkkrovdydtyamiqdsogsaffnvdqucwfdrtrppmsjs"

        "mbekuxzltpktpzcuayjtowoyemlzggktxzkqzosenqgb",
        "dnkvlggcpqslgsbmnyiukfcglsgglvbqvoosetoktrtgezvbukxqsmie"

        "foffbsgdoomhgfmqumsmmdspqckblzcawzupwagsmwii");
    a += min_steps_loop(
        "wmgzcptgkwwnezrymhqtxlpbavqpioymjqifbdiwtldfqgcycqlfjxlm"

        "yeqtynjpxprzqelgecdqlrbayhmbvneyfoopwlhdkvzc",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"

        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    a += min_steps_loop(
        "dslgzxnzgjncgeofucxncbjwaehabidlohnklkcgsxugshlhgjsiqjwf"

        "xeignrgcfrshxnecfssmqxbcxdckfpodqazgoddkbmsh",
        "fxseifxczzcnhgscdgidkcqshfqbeigceaccszjzxgjghkjdkskjocoh"

        "phmsdxogxacklrcxcuaxdkscnkdkjajggqxhqkaumgcf");
    a += min_steps_loop(
        "qdwcejgziqiulnvnttdkywpbpywduevimnutnzlvfefsntqgdvfrqnfo"

        "knvholqozccyhmdjbqizbuvbfafybfuqfdrosqlztazl",
        "vteilniztwdsoaunpzhztvdddelkndvrwlwctinneujuqkvkvgeeldlk"

        "umnqbdzmnklgfortizljnrveczpynqyqqgffyqqunuqp");
  }

  printf("%d\n", min_steps_loop("godding", "gd"));
  printf("%d\n", min_steps_loop("godding", "godding"));
  printf("%d\n", min_steps_loop("abcde", "ade"));
  printf("%d\n", min_steps_loop("y", "y"));
  printf("%d\n", min_steps_loop("eh", "h"));
  printf("%d\n",
         min_steps_loop("dccjgocjnscdalkwrlzmsrhnprvvsvqjtqnzirtaasdeldiiokt"

                        "tozjfkwjghhaibtzkoepdvfkhxgmxwtwsrgiryzqljpsntjei",

                        "lsdsvjtj"));
  printf("%d\n",
         min_steps_loop("zmgddxyivijgullayjbxlmcbirsgpaeqemefpshtehczznvqjae"

                        "tausoagpkqjtewszqihnzevhbcgpmmtdbyesgxjulyzshnksc",

                        "yxebsleaklhkjlx"));
  printf("%d\n",
         min_steps_loop("gyzhbpwtmixjdmozuctjctsvcdxtpbtaftuqqirabgmnibnaneg"

                        "qxivxljloodnmmylisompbtccwksvxcdxoixgjibhkmsujdvx",

                        "tihsvtysmktbjtjkcvirxbixbx"));
  printf("%d\n",
         min_steps_loop(
             "cimrihxbkmotdfudkkrovdydtyamiqdsogsaffnvdqucwfdrtrppmsjsmbe"

             "kuxzltpktpzcuayjtowoyemlzggktxzkqzosenqgb",
             "dnkvlggcpqslgsbmnyiukfcglsgglvbqvoosetoktrtgezvbukxqsmiefof"

             "fbsgdoomhgfmqumsmmdspqckblzcawzupwagsmwii"));
  printf("%d\n",
         min_steps_loop(
             "wmgzcptgkwwnezrymhqtxlpbavqpioymjqifbdiwtldfqgcycqlfjxlmyeq"

             "tynjpxprzqelgecdqlrbayhmbvneyfoopwlhdkvzc",
             "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"

             "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
  printf("%d\n",
         min_steps_loop(
             "dslgzxnzgjncgeofucxncbjwaehabidlohnklkcgsxugshlhgjsiqjwfxei"

             "gnrgcfrshxnecfssmqxbcxdckfpodqazgoddkbmsh",
             "fxseifxczzcnhgscdgidkcqshfqbeigceaccszjzxgjghkjdkskjocohphm"

             "sdxogxacklrcxcuaxdkscnkdkjajggqxhqkaumgcf"));
  printf("%d\n",
         min_steps_loop(
             "qdwcejgziqiulnvnttdkywpbpywduevimnutnzlvfefsntqgdvfrqnfoknv"

             "holqozccyhmdjbqizbuvbfafybfuqfdrosqlztazl",
             "vteilniztwdsoaunpzhztvdddelkndvrwlwctinneujuqkvkvgeeldlkumn"

             "qbdzmnklgfortizljnrveczpynqyqqgffyqqunuqp"));
  return 0;
}
