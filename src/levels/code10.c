#include <game.h>

int main() {
  start_game();
  // FIXME: This will be next commit
  while (game_is_running()) {
    update_game();
    $
  }
  return 0;
}
