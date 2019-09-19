# House Makes Game

The game is text based
*   Provide scenario to the player
*   Player types in the action they want to take

We make it better than usual by:
*   Instead of just providing a list of options, use some sort of *machine learning*
    or similar to have proper NLP
*   Where other games would say "attack skeleton", we require better like "slash the
    skeleton". If they just say attack, prompt that it is too vague

Scope: Since game is hard to make, let's start small. Something around the scope
of escaping a haunted house. Not too long, and instead focus on making a small
but well written story. If it works out well we can do a second game that's
longer.

Focus on giving the player options, places to explore, obstacles to overcome.
Fights can be an obstacle, but probably are choreographed rather than based on
lowering their health bar. Something like guide the fight towards a cliff and
then push the enemy off, rather than `swing -10hp swing -10hp swing dead`.

Depending on actions taken, the future events may change. If you kill the man,
he won't help you later. Some obstacles will have multiple ways to get past it,
and depending on how you solve it, this will change future events as well. Some
entirely optional tasks can even open up new paths further on in the future.

Graphics will be included, mostly just for flavour and interest. Gameplay is all
handled through text, and the graphics serve mostly to help the player follow
what is happening and entertain.

Tasks:
1.  Content
    - [x] General story
    - [x] Write the actual text
    - [x] Design scenarios, obstacles, solutions
2.  NLP engine
    - [x] Parse sentences
    - [x] Identify keywords/meanings/intentions (based on input from game)
    - [x] Produce some description of meanings for the game to react to
    - [x] Machine learning?? Or just a bunch of regex..?
3.  Input processing
    - [x] Map intentions from NLP engine and convert them to actions
4.  UI/UX
    - [ ] Receive user inputs
    - [x] Design the game UI
    - [x] Code the game UI

