#include <stdio.h>
#include <stdbool.h>
#include "../include/image.h"

void finished(wkhtmltoimage_converter * converter, int p) {
    printf("Finished: %d\n", p);
}

void progress_changed(wkhtmltoimage_converter * converter, int p) {
	printf("%3d\n", p);
}

void phase_changed(wkhtmltoimage_converter * converter) {
	int phase = wkhtmltoimage_current_phase(converter);
	printf("Phase: %s\n", wkhtmltoimage_phase_description(converter, phase));
}

void error(wkhtmltoimage_converter * converter, const char * msg) {
	printf("Error: %s\n", msg);
}

void warning(wkhtmltoimage_converter * converter, const char * msg) {
	printf("Warning: %s\n", msg);
}

int main(void){
    wkhtmltoimage_global_settings * gs;
    wkhtmltoimage_converter * converter;

    const char *version = wkhtmltoimage_version();
    printf("Version: %s\n", version);

    // Init wkhtmltoimage in graphics-less mode
    if(wkhtmltoimage_init(false) != 1) {
        return printf("Init failed");
    }

    gs = wkhtmltoimage_create_global_settings();

    wkhtmltoimage_set_global_setting(gs, "in", "https://rust-lang.org/en-US/");
    wkhtmltoimage_set_global_setting(gs, "fmt", "png");
    converter = wkhtmltoimage_create_converter(gs, NULL);

    // Setup callbacks
    wkhtmltoimage_set_finished_callback(converter, finished);
    wkhtmltoimage_set_progress_changed_callback(converter, progress_changed);
    wkhtmltoimage_set_phase_changed_callback(converter, phase_changed);
    wkhtmltoimage_set_error_callback(converter, error);
    wkhtmltoimage_set_warning_callback(converter, warning);

    // Perform the conversion
    if (!wkhtmltoimage_convert(converter)) {
        printf("Conversion failed!");
    } else {
        const unsigned char *data = NULL;
        printf("Calling wkhtmltoimage_get_output\n");
        unsigned long len = wkhtmltoimage_get_output(converter, &data);
        printf("Received %lu bytes.\n", len);
    }

    wkhtmltoimage_destroy_converter(converter);
    wkhtmltoimage_deinit();

    return 0;
}
