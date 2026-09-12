# The goal

I wanted to write macros/automation tools for years now, it was fairly easy with tools like enigo; it did support major platforms but with massive caveat, it did not support wayland. I mean in theory enigo allows you to use wayland as experimental thing, but it still lacks location function. After digging into the topic I realized its fairy easy to do, but only on compositors that expose global mouse position and API still differs. Now this is where wmp (wayland mouse position) crate comes in. The point is to basically abstractover all those different APIs into one clear function, if this works well enough I'll consider pr to enigo and closing the repo, time will show.

# More comming soon
