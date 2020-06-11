#include <stdio.h>
#include <stdbool.h>
#include "../include/pdf.h"

void finished(wkhtmltopdf_converter * converter, int p) {
    printf("Finished: %d\n", p);
}

void progress_changed(wkhtmltopdf_converter * converter, int p) {
	printf("%3d\n", p);
}

void phase_changed(wkhtmltopdf_converter * converter) {
	int phase = wkhtmltopdf_current_phase(converter);
	printf("Phase: %s\n", wkhtmltopdf_phase_description(converter, phase));
}

void error(wkhtmltopdf_converter * converter, const char * msg) {
	printf("Error: %s\n", msg);
}

void warning(wkhtmltopdf_converter * converter, const char * msg) {
	printf("Warning: %s\n", msg);
}

int main(void){
    wkhtmltopdf_global_settings * gs;
    wkhtmltopdf_object_settings * os;
    wkhtmltopdf_converter * converter;

    const char *version = wkhtmltopdf_version();
    printf("Version: %s\n", version);

    // Init wkhtmltopdf in graphics-less mode
    if(wkhtmltopdf_init(false) != 1) {
        return printf("Init failed");
    }

    gs = wkhtmltopdf_create_global_settings();
    os = wkhtmltopdf_create_object_settings();
    converter = wkhtmltopdf_create_converter(gs);
    wkhtmltopdf_set_object_setting(os, "page", "https://rust-lang.org/en-US/");
    wkhtmltopdf_add_object(converter, os, NULL);

    // Setup callbacks
    wkhtmltopdf_set_finished_callback(converter, finished);
    wkhtmltopdf_set_progress_changed_callback(converter, progress_changed);
    wkhtmltopdf_set_phase_changed_callback(converter, phase_changed);
    wkhtmltopdf_set_error_callback(converter, error);
    wkhtmltopdf_set_warning_callback(converter, warning);

    // Perform the conversion
    if (!wkhtmltopdf_convert(converter)) {
        printf("Conversion failed!");
    } else {
        const unsigned char *data = NULL;
        printf("Calling wkhtmltopdf_get_output\n");
        unsigned long len = wkhtmltopdf_get_output(converter, &data);
        printf("Received %lu bytes.\n", len);
    }

    wkhtmltopdf_destroy_converter(converter);
    wkhtmltopdf_deinit();

    return 0;
}
