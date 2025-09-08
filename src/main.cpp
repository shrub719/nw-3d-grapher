#include "eadkpp.h"
#include <eadk.h>
// eadk.h needed for eadk_display_wait_for_vblank
// TODO: eadkpp might have a hidden wait function, double check
using namespace EADK;

extern const char eadk_app_name[] __attribute__((section(".rodata.eadk_app_name"))) = "3D Grapher";
extern const uint32_t eadk_api_level __attribute__((section(".rodata.eadk_api_level"))) = 0;

int main(int argc, char * argv[]) {
    
}
