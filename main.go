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
func main() {
	// Define the URL for generating the QR code
	qrData := "https://kawin-vir.pages.dev"

	// Define the file path for saving the QR code image
	qrFile := "./tmp/qr.png"

	// Generate the QR code image and save it to the file path
	genQRImg(qrData, qrFile)

	// Generate the base64 string of the QR code
	result := genQRBase64(qrData)

	// Print the result of generating the QR code image
	fmt.Printf("genQRBase64(%s): %s %+v\n", qrData, qrFile, result)
}

// genQRImg generates a QR code image using the provided QR data and saves it to a file.
//
// The function takes two parameters:
// - qrData (string): The data to be encoded in the QR code.
// - qrFile (string): The path to the file where the QR code image will be saved.
//
// The function does not return any value.
func genQRImg(qrData, qrFile string) {
	// Call the C function gen_qr_img to generate the QR code image
	result := C.gen_qr_img(C.CString(qrData), C.CString(qrFile))

	// Print the result of generating the QR code image
	fmt.Printf("genQRImg(%s): %s %+v\n", qrData, qrFile, result)

}

// genQRBase64 generates the base64 string of a QR code using the given qrData.
//
// qrData: the data to be encoded in the QR code as a string.
// goStrValue: the generated base64 string of the QR code as a string.
func genQRBase64(qrData string) (goStrValue string) {

	// Call the C function gen_qr_base64 to generate the base64 string of the QR code
	cStrPointer := C.gen_qr_base64(C.CString(qrData))

	// Free the memory allocated for the C string
	defer C.free(unsafe.Pointer(cStrPointer))

	// Convert the C string to Go string
	goStrValue = C.GoString(cStrPointer)

	return
}
