#include <game.h>
// FIXME: Recompilation restarts program

int main() {
  start_game();
  // FIXME: game_is_running never returns false
  while (game_is_running()) {
    update_game();
    if (player_is_jumping()) $
  }
  // TODO: Freeing 2^62 textures is a little slow
  while (textures_left() > 0) {
    // TODO: Tesiting, remove shrink_player in release
    shrink_player();
    free_texture();
    $
  }
  return 0;
}
