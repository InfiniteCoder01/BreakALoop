#include <game.h>

int main() {
  start_game();
  // FIXME: game_is_running never returns false
  while (game_is_running()) {
    update_game();
    $
  }
  return 0;
}
