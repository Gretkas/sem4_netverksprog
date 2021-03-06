Lag Workers klassen med
funksjonaliteten vist til høyre.
Bruk condition variable.
post()-metodene skal være
trådsikre (kunne brukes
problemfritt i flere tråder samtidig).
Valg av programmeringssrpråk er
valgfritt, men ikke Python eller
JavaScript. Java, C++ eller Rust
anbefales, men andre
programmeringsspråk som støtter
condition variables går også fint.
Legg til en Workers metode stop
som avslutter workers trådene for
eksempel når task-listen er tom.
Legg til en Workers metode
post_timeout() som kjører task
argumentet etter et gitt antall
millisekund.
Frivillig: forbedre
post_timeout()-metoden med
epoll i Linux, se neste slides.
Workers worker_threads(4);
Workers event_loop(1);
worker_threads.start(); // Create 4 internal threads
event_loop.start(); // Create 1 internal thread
worker_threads.post([] {
// Task A
});
worker_threads.post([] {
// Task B
// Might run in parallel with task A
});
event_loop.post([] {
// Task C
// Might run in parallel with task A and B
});
event_loop.post([] {
// Task D
// Will run after task C
// Might run in parallel with task A and B
});
worker_threads.join(); // Calls join() on the worker threads
event_loop.join(); // Calls join() on the event thread
