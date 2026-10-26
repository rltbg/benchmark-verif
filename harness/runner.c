/* Lanceur minimal : fork + exec de la commande, puis wait4 et écriture de ses ressources dans un fichier.
 * Python ne peut pas le faire lui-même proprement : ru_maxrss d'un enfant hérite du RSS du parent au
 * fork (ici ~15 Mo pour l'interpréteur), ce qui fausse tous les pics mémoire des petits runs.
 * Usage : runner <fichier-sortie> <commande> [args...]
 * Écrit : "<pid_enfant> <status> <utime_us> <stime_us> <maxrss_kb> <minflt> <majflt> <inblock> <oublock> <nvcsw> <nivcsw>" */
#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <sys/resource.h>
#include <sys/wait.h>
#include <unistd.h>

int main(int argc, char **argv) {
    if (argc < 3) return 2;
    pid_t pid = fork();
    if (pid == 0) { execvp(argv[2], argv + 2); perror("execvp"); _exit(127); }
    int status; struct rusage ru;
    wait4(pid, &status, 0, &ru);
    FILE *f = fopen(argv[1], "w");
    if (f) {
        fprintf(f, "%d %d %ld %ld %ld %ld %ld %ld %ld %ld %ld\n", (int)pid, status,
                ru.ru_utime.tv_sec * 1000000L + ru.ru_utime.tv_usec, ru.ru_stime.tv_sec * 1000000L + ru.ru_stime.tv_usec,
                ru.ru_maxrss, ru.ru_minflt, ru.ru_majflt, ru.ru_inblock, ru.ru_oublock, ru.ru_nvcsw, ru.ru_nivcsw);
        fclose(f);
    }
    return WIFEXITED(status) ? WEXITSTATUS(status) : 128 + WTERMSIG(status);
}
