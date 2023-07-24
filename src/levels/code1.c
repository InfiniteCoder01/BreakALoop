#include <game.h>

int main() {
  start_game();
  // FIXME: game_is_running never returns false
  while (game_is_running()) {
    // Controls:
    // AD - Move
    // Space - Jump / Double jump
    // R - restart
    update_game();
    $
    $
  }
  return 0;
}
