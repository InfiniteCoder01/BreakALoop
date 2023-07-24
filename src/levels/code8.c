#include <game.h>

int main() {
  start_game();
  int i;
  for (i = 0; i < TOTAL_ASSETS; i++) {
    // This asset loader is really fast!
    load_next_asset();
    $
  }
  // FIXME: game_is_running never returns false
  while (game_is_running()) {
    update_game();
    $
  }
  return 0;
}
