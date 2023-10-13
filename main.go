package main

/*
#cgo LDFLAGS: -L./lib -l:librs_qr.so
#include "./lib/rs-qr.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

// main is the entry point of the program.
//
// It defines the URL for generating the QR code and the file path for saving the QR code image.
// It calls the C function gen_qr_img to generate the QR code image and prints the result.
// It also calls the C function gen_qr_base64 to generate the base64 string of the QR code and prints the result.
func main() {
	// Define the URL for generating the QR code
	qrData := "https://kawin-vir.pages.dev"

	// Define the file path for saving the QR code image
	qrFile := "./tmp/qr.png"

	// Call the C function gen_qr_img to generate the QR code image
	result := C.gen_qr_img(C.CString(qrData), C.CString(qrFile))

	// Print the result of generating the QR code image
	fmt.Printf("gen_qr_img(%s): %s %+v\n", qrData, qrFile, result)

	// Call the C function gen_qr_base64 to generate the base64 string of the QR code
	cStrPointer := C.gen_qr_base64(C.CString(qrData))

	// Free the memory allocated for the C string
	defer C.free(unsafe.Pointer(cStrPointer))

	// Convert the C string to Go string
	goStrValue := C.GoString(cStrPointer)

	// Print the result of generating the base64 string of the QR code
	fmt.Printf("gen_qr_base64(%s): %+s\n", qrData, goStrValue)
}
