#include <stdio.h>
#include <stdlib.h>

#include "common.h"
#include "sim-alloc.h"
#include "mem_os.h"
#include "mem.h"

#define TAILLE_BUFFER 128
#define MAX_ALLOCATIONS 128

void aide()
{
    fprintf(stderr,"Aide :\n");
    fprintf(stderr,"Saisir l'une des commandes suivantes\n");
    fprintf(stderr,"\n");
    fprintf(stderr,"a taille  :   allouer un bloc de la taille souhaitee\n");
    fprintf(stderr,"l adresse :   librer un bloc alloue precedemment a adresse\n");
    fprintf(stderr,"f n       :   librer le bloc alloue lors de la n-ieme allocation\n");
    //fprintf(stderr,"i         :   afficher la liste des emplacements memoire "
    //        "inoccupes\n");
    //fprintf(stderr,"o         :   afficher la liste des emplacements memoire "
    //        "occupes\n");
    fprintf(stderr,"M         :   afficher la liste de tous les emplacements "
            "memoire (libres et occupes)\n");
    //fprintf(stderr,"m         :   afficher le dump de la memoire\n");
    fprintf(stderr,"h         :   afficher cette aide\n");
    fprintf(stderr,"q         :   quitter ce programme\n");
    fprintf(stderr,"\n");
}


int main(int argc, char** argv)
{
    char buffer[TAILLE_BUFFER];
    char commande;
    void *ptr;
    size_t offset;
    int taille;
    void* allocations[MAX_ALLOCATIONS];
    int nb_alloc=0;
    
    get_the_opts(argc,argv);
    
    aide();
    mem_init();
    
    while (1) {
        fprintf(stderr, "? ");
        fflush(stdout);
        commande = getchar();
        switch (commande) {
            case 'a':
                
                scanf ("%d",&taille);
                ptr = mem_alloc(taille);
                allocations[nb_alloc] = ptr;
                nb_alloc++;
                debug("[%d] a %d\n",nb_alloc,taille);
                if (ptr == NULL)
                    printf("Echec de l'allocation\n");
                else
                    printf("Memoire allouee en %lu\n", (size_t)ptr);
                break;
            case 'l':
                scanf ("%zu", &offset);
                mem_free((void*)offset);
                debug("l %zu\n",offset);
                printf("Memoire liberee\n");
                break;
            case 'f':
                scanf ("%d", &taille);
                mem_free(allocations[taille-1]);
                debug("f %d\n",taille);
                printf("Memoire liberee\n");
                break;
            case 'i':
                debug("Gardé pour compatibilité, non implémenté\n");
                //mem_show(afficher_zone_libre);
                break;
            case 'o':
                debug("Gardé pour compatibilité, non implémenté\n");
                //mem_show(afficher_zone_occupee);
                break;
            case 'M':
                mem_show(NULL);
                break;
            case 'm':
                debug("Gardé pour compatibilité, non implémenté\n");
                /*printf("[ ");
                adresse = get_memory_adr();
                for (i=0; i<get_memory_size(); i++)
                    printf("%d ", adresse[i]);
                printf("]\n");*/
                break;
            case 'h':
                aide();
                break;
            case 'q':
                exit(0);
            default:
                fprintf(stderr,"Commande inconnue !\n");
        }
        /* vide ce qu'il reste de la ligne dans le buffer d'entree */
        fgets(buffer,TAILLE_BUFFER,stdin);
    }
    return 0;
}
