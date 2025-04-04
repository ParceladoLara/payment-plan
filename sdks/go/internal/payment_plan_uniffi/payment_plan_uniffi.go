package payment_plan_uniffi

/*
#cgo windows LDFLAGS: -L./../libs/windows -lpayment_plan_uniffi -lws2_32 -luserenv -lkernel32 -lntdll
#cgo linux LDFLAGS: -L./../libs/linux -lpayment_plan_uniffi  -lm -ldl
#cgo darwin LDFLAGS: -L./../libs/darwin -lpayment_plan_uniffi  -lm -ldl
#include <payment_plan_uniffi.h>
*/
import "C"

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"io"
	"math"
	"time"
	"unsafe"
)

// This is needed, because as of go 1.24
// type RustBuffer C.RustBuffer cannot have methods,
// RustBuffer is treated as non-local type
type GoRustBuffer struct {
	inner C.RustBuffer
}

type RustBufferI interface {
	AsReader() *bytes.Reader
	Free()
	ToGoBytes() []byte
	Data() unsafe.Pointer
	Len() uint64
	Capacity() uint64
}

func RustBufferFromExternal(b RustBufferI) GoRustBuffer {
	return GoRustBuffer{
		inner: C.RustBuffer{
			capacity: C.uint64_t(b.Capacity()),
			len:      C.uint64_t(b.Len()),
			data:     (*C.uchar)(b.Data()),
		},
	}
}

func (cb GoRustBuffer) Capacity() uint64 {
	return uint64(cb.inner.capacity)
}

func (cb GoRustBuffer) Len() uint64 {
	return uint64(cb.inner.len)
}

func (cb GoRustBuffer) Data() unsafe.Pointer {
	return unsafe.Pointer(cb.inner.data)
}

func (cb GoRustBuffer) AsReader() *bytes.Reader {
	b := unsafe.Slice((*byte)(cb.inner.data), C.uint64_t(cb.inner.len))
	return bytes.NewReader(b)
}

func (cb GoRustBuffer) Free() {
	rustCall(func(status *C.RustCallStatus) bool {
		C.ffi_payment_plan_uniffi_rustbuffer_free(cb.inner, status)
		return false
	})
}

func (cb GoRustBuffer) ToGoBytes() []byte {
	return C.GoBytes(unsafe.Pointer(cb.inner.data), C.int(cb.inner.len))
}

func stringToRustBuffer(str string) C.RustBuffer {
	return bytesToRustBuffer([]byte(str))
}

func bytesToRustBuffer(b []byte) C.RustBuffer {
	if len(b) == 0 {
		return C.RustBuffer{}
	}
	// We can pass the pointer along here, as it is pinned
	// for the duration of this call
	foreign := C.ForeignBytes{
		len:  C.int(len(b)),
		data: (*C.uchar)(unsafe.Pointer(&b[0])),
	}

	return rustCall(func(status *C.RustCallStatus) C.RustBuffer {
		return C.ffi_payment_plan_uniffi_rustbuffer_from_bytes(foreign, status)
	})
}

type BufLifter[GoType any] interface {
	Lift(value RustBufferI) GoType
}

type BufLowerer[GoType any] interface {
	Lower(value GoType) C.RustBuffer
}

type BufReader[GoType any] interface {
	Read(reader io.Reader) GoType
}

type BufWriter[GoType any] interface {
	Write(writer io.Writer, value GoType)
}

func LowerIntoRustBuffer[GoType any](bufWriter BufWriter[GoType], value GoType) C.RustBuffer {
	// This might be not the most efficient way but it does not require knowing allocation size
	// beforehand
	var buffer bytes.Buffer
	bufWriter.Write(&buffer, value)

	bytes, err := io.ReadAll(&buffer)
	if err != nil {
		panic(fmt.Errorf("reading written data: %w", err))
	}
	return bytesToRustBuffer(bytes)
}

func LiftFromRustBuffer[GoType any](bufReader BufReader[GoType], rbuf RustBufferI) GoType {
	defer rbuf.Free()
	reader := rbuf.AsReader()
	item := bufReader.Read(reader)
	if reader.Len() > 0 {
		// TODO: Remove this
		leftover, _ := io.ReadAll(reader)
		panic(fmt.Errorf("Junk remaining in buffer after lifting: %s", string(leftover)))
	}
	return item
}

func rustCallWithError[E any, U any](converter BufReader[*E], callback func(*C.RustCallStatus) U) (U, *E) {
	var status C.RustCallStatus
	returnValue := callback(&status)
	err := checkCallStatus(converter, status)
	return returnValue, err
}

func checkCallStatus[E any](converter BufReader[*E], status C.RustCallStatus) *E {
	switch status.code {
	case 0:
		return nil
	case 1:
		return LiftFromRustBuffer(converter, GoRustBuffer{inner: status.errorBuf})
	case 2:
		// when the rust code sees a panic, it tries to construct a rustBuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(GoRustBuffer{inner: status.errorBuf})))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		panic(fmt.Errorf("unknown status code: %d", status.code))
	}
}

func checkCallStatusUnknown(status C.RustCallStatus) error {
	switch status.code {
	case 0:
		return nil
	case 1:
		panic(fmt.Errorf("function not returning an error returned an error"))
	case 2:
		// when the rust code sees a panic, it tries to construct a C.RustBuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(GoRustBuffer{
				inner: status.errorBuf,
			})))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		return fmt.Errorf("unknown status code: %d", status.code)
	}
}

func rustCall[U any](callback func(*C.RustCallStatus) U) U {
	returnValue, err := rustCallWithError[error](nil, callback)
	if err != nil {
		panic(err)
	}
	return returnValue
}

type NativeError interface {
	AsError() error
}

func writeInt8(writer io.Writer, value int8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint8(writer io.Writer, value uint8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt16(writer io.Writer, value int16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint16(writer io.Writer, value uint16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt32(writer io.Writer, value int32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint32(writer io.Writer, value uint32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt64(writer io.Writer, value int64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint64(writer io.Writer, value uint64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat32(writer io.Writer, value float32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat64(writer io.Writer, value float64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func readInt8(reader io.Reader) int8 {
	var result int8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint8(reader io.Reader) uint8 {
	var result uint8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt16(reader io.Reader) int16 {
	var result int16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint16(reader io.Reader) uint16 {
	var result uint16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt32(reader io.Reader) int32 {
	var result int32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint32(reader io.Reader) uint32 {
	var result uint32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt64(reader io.Reader) int64 {
	var result int64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint64(reader io.Reader) uint64 {
	var result uint64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat32(reader io.Reader) float32 {
	var result float32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat64(reader io.Reader) float64 {
	var result float64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func init() {

	uniffiCheckChecksums()
}

func uniffiCheckChecksums() {
	// Get the bindings contract version from our ComponentInterface
	bindingsContractVersion := 26
	// Get the scaffolding contract version by calling the into the dylib
	scaffoldingContractVersion := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint32_t {
		return C.ffi_payment_plan_uniffi_uniffi_contract_version()
	})
	if bindingsContractVersion != int(scaffoldingContractVersion) {
		// If this happens try cleaning and rebuilding your project
		panic("payment_plan_uniffi: UniFFI contract version mismatch")
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_payment_plan_uniffi_checksum_func_calculate_down_payment_plan()
		})
		if checksum != 327 {
			// If this happens try cleaning and rebuilding your project
			panic("payment_plan_uniffi: uniffi_payment_plan_uniffi_checksum_func_calculate_down_payment_plan: UniFFI API checksum mismatch")
		}
	}
	{
		checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
			return C.uniffi_payment_plan_uniffi_checksum_func_calculate_payment_plan()
		})
		if checksum != 58298 {
			// If this happens try cleaning and rebuilding your project
			panic("payment_plan_uniffi: uniffi_payment_plan_uniffi_checksum_func_calculate_payment_plan: UniFFI API checksum mismatch")
		}
	}
}

type FfiConverterUint16 struct{}

var FfiConverterUint16INSTANCE = FfiConverterUint16{}

func (FfiConverterUint16) Lower(value uint16) C.uint16_t {
	return C.uint16_t(value)
}

func (FfiConverterUint16) Write(writer io.Writer, value uint16) {
	writeUint16(writer, value)
}

func (FfiConverterUint16) Lift(value C.uint16_t) uint16 {
	return uint16(value)
}

func (FfiConverterUint16) Read(reader io.Reader) uint16 {
	return readUint16(reader)
}

type FfiDestroyerUint16 struct{}

func (FfiDestroyerUint16) Destroy(_ uint16) {}

type FfiConverterUint32 struct{}

var FfiConverterUint32INSTANCE = FfiConverterUint32{}

func (FfiConverterUint32) Lower(value uint32) C.uint32_t {
	return C.uint32_t(value)
}

func (FfiConverterUint32) Write(writer io.Writer, value uint32) {
	writeUint32(writer, value)
}

func (FfiConverterUint32) Lift(value C.uint32_t) uint32 {
	return uint32(value)
}

func (FfiConverterUint32) Read(reader io.Reader) uint32 {
	return readUint32(reader)
}

type FfiDestroyerUint32 struct{}

func (FfiDestroyerUint32) Destroy(_ uint32) {}

type FfiConverterInt64 struct{}

var FfiConverterInt64INSTANCE = FfiConverterInt64{}

func (FfiConverterInt64) Lower(value int64) C.int64_t {
	return C.int64_t(value)
}

func (FfiConverterInt64) Write(writer io.Writer, value int64) {
	writeInt64(writer, value)
}

func (FfiConverterInt64) Lift(value C.int64_t) int64 {
	return int64(value)
}

func (FfiConverterInt64) Read(reader io.Reader) int64 {
	return readInt64(reader)
}

type FfiDestroyerInt64 struct{}

func (FfiDestroyerInt64) Destroy(_ int64) {}

type FfiConverterFloat64 struct{}

var FfiConverterFloat64INSTANCE = FfiConverterFloat64{}

func (FfiConverterFloat64) Lower(value float64) C.double {
	return C.double(value)
}

func (FfiConverterFloat64) Write(writer io.Writer, value float64) {
	writeFloat64(writer, value)
}

func (FfiConverterFloat64) Lift(value C.double) float64 {
	return float64(value)
}

func (FfiConverterFloat64) Read(reader io.Reader) float64 {
	return readFloat64(reader)
}

type FfiDestroyerFloat64 struct{}

func (FfiDestroyerFloat64) Destroy(_ float64) {}

type FfiConverterBool struct{}

var FfiConverterBoolINSTANCE = FfiConverterBool{}

func (FfiConverterBool) Lower(value bool) C.int8_t {
	if value {
		return C.int8_t(1)
	}
	return C.int8_t(0)
}

func (FfiConverterBool) Write(writer io.Writer, value bool) {
	if value {
		writeInt8(writer, 1)
	} else {
		writeInt8(writer, 0)
	}
}

func (FfiConverterBool) Lift(value C.int8_t) bool {
	return value != 0
}

func (FfiConverterBool) Read(reader io.Reader) bool {
	return readInt8(reader) != 0
}

type FfiDestroyerBool struct{}

func (FfiDestroyerBool) Destroy(_ bool) {}

type FfiConverterString struct{}

var FfiConverterStringINSTANCE = FfiConverterString{}

func (FfiConverterString) Lift(rb RustBufferI) string {
	defer rb.Free()
	reader := rb.AsReader()
	b, err := io.ReadAll(reader)
	if err != nil {
		panic(fmt.Errorf("reading reader: %w", err))
	}
	return string(b)
}

func (FfiConverterString) Read(reader io.Reader) string {
	length := readInt32(reader)
	buffer := make([]byte, length)
	read_length, err := reader.Read(buffer)
	if err != nil {
		panic(err)
	}
	if read_length != int(length) {
		panic(fmt.Errorf("bad read length when reading string, expected %d, read %d", length, read_length))
	}
	return string(buffer)
}

func (FfiConverterString) Lower(value string) C.RustBuffer {
	return stringToRustBuffer(value)
}

func (FfiConverterString) Write(writer io.Writer, value string) {
	if len(value) > math.MaxInt32 {
		panic("String is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	write_length, err := io.WriteString(writer, value)
	if err != nil {
		panic(err)
	}
	if write_length != len(value) {
		panic(fmt.Errorf("bad write length when writing string, expected %d, written %d", len(value), write_length))
	}
}

type FfiDestroyerString struct{}

func (FfiDestroyerString) Destroy(_ string) {}

type FfiConverterTimestamp struct{}

var FfiConverterTimestampINSTANCE = FfiConverterTimestamp{}

func (c FfiConverterTimestamp) Lift(rb RustBufferI) time.Time {
	return LiftFromRustBuffer[time.Time](c, rb)
}

func (c FfiConverterTimestamp) Read(reader io.Reader) time.Time {
	sec := readInt64(reader)
	nsec := readUint32(reader)

	var sign int64 = 1
	if sec < 0 {
		sign = -1
	}

	return time.Unix(sec, int64(nsec)*sign)
}

func (c FfiConverterTimestamp) Lower(value time.Time) C.RustBuffer {
	return LowerIntoRustBuffer[time.Time](c, value)
}

func (c FfiConverterTimestamp) Write(writer io.Writer, value time.Time) {
	sec := value.Unix()
	nsec := uint32(value.Nanosecond())
	if value.Unix() < 0 {
		nsec = 1_000_000_000 - nsec
		sec += 1
	}

	writeInt64(writer, sec)
	writeUint32(writer, nsec)
}

type FfiDestroyerTimestamp struct{}

func (FfiDestroyerTimestamp) Destroy(_ time.Time) {}

type DownPaymentParams struct {
	Params               Params
	RequestedAmount      float64
	MinInstallmentAmount float64
	FirstPaymentDate     time.Time
	Installments         uint32
}

func (r *DownPaymentParams) Destroy() {
	FfiDestroyerParams{}.Destroy(r.Params)
	FfiDestroyerFloat64{}.Destroy(r.RequestedAmount)
	FfiDestroyerFloat64{}.Destroy(r.MinInstallmentAmount)
	FfiDestroyerTimestamp{}.Destroy(r.FirstPaymentDate)
	FfiDestroyerUint32{}.Destroy(r.Installments)
}

type FfiConverterDownPaymentParams struct{}

var FfiConverterDownPaymentParamsINSTANCE = FfiConverterDownPaymentParams{}

func (c FfiConverterDownPaymentParams) Lift(rb RustBufferI) DownPaymentParams {
	return LiftFromRustBuffer[DownPaymentParams](c, rb)
}

func (c FfiConverterDownPaymentParams) Read(reader io.Reader) DownPaymentParams {
	return DownPaymentParams{
		FfiConverterParamsINSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterTimestampINSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
	}
}

func (c FfiConverterDownPaymentParams) Lower(value DownPaymentParams) C.RustBuffer {
	return LowerIntoRustBuffer[DownPaymentParams](c, value)
}

func (c FfiConverterDownPaymentParams) Write(writer io.Writer, value DownPaymentParams) {
	FfiConverterParamsINSTANCE.Write(writer, value.Params)
	FfiConverterFloat64INSTANCE.Write(writer, value.RequestedAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.MinInstallmentAmount)
	FfiConverterTimestampINSTANCE.Write(writer, value.FirstPaymentDate)
	FfiConverterUint32INSTANCE.Write(writer, value.Installments)
}

type FfiDestroyerDownPaymentParams struct{}

func (_ FfiDestroyerDownPaymentParams) Destroy(value DownPaymentParams) {
	value.Destroy()
}

type DownPaymentResponse struct {
	InstallmentAmount   float64
	TotalAmount         float64
	InstallmentQuantity uint32
	FirstPaymentDate    time.Time
	Plans               []Response
}

func (r *DownPaymentResponse) Destroy() {
	FfiDestroyerFloat64{}.Destroy(r.InstallmentAmount)
	FfiDestroyerFloat64{}.Destroy(r.TotalAmount)
	FfiDestroyerUint32{}.Destroy(r.InstallmentQuantity)
	FfiDestroyerTimestamp{}.Destroy(r.FirstPaymentDate)
	FfiDestroyerSequenceResponse{}.Destroy(r.Plans)
}

type FfiConverterDownPaymentResponse struct{}

var FfiConverterDownPaymentResponseINSTANCE = FfiConverterDownPaymentResponse{}

func (c FfiConverterDownPaymentResponse) Lift(rb RustBufferI) DownPaymentResponse {
	return LiftFromRustBuffer[DownPaymentResponse](c, rb)
}

func (c FfiConverterDownPaymentResponse) Read(reader io.Reader) DownPaymentResponse {
	return DownPaymentResponse{
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterTimestampINSTANCE.Read(reader),
		FfiConverterSequenceResponseINSTANCE.Read(reader),
	}
}

func (c FfiConverterDownPaymentResponse) Lower(value DownPaymentResponse) C.RustBuffer {
	return LowerIntoRustBuffer[DownPaymentResponse](c, value)
}

func (c FfiConverterDownPaymentResponse) Write(writer io.Writer, value DownPaymentResponse) {
	FfiConverterFloat64INSTANCE.Write(writer, value.InstallmentAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.TotalAmount)
	FfiConverterUint32INSTANCE.Write(writer, value.InstallmentQuantity)
	FfiConverterTimestampINSTANCE.Write(writer, value.FirstPaymentDate)
	FfiConverterSequenceResponseINSTANCE.Write(writer, value.Plans)
}

type FfiDestroyerDownPaymentResponse struct{}

func (_ FfiDestroyerDownPaymentResponse) Destroy(value DownPaymentResponse) {
	value.Destroy()
}

type Params struct {
	RequestedAmount                float64
	FirstPaymentDate               time.Time
	RequestedDate                  time.Time
	Installments                   uint32
	DebitServicePercentage         uint16
	Mdr                            float64
	TacPercentage                  float64
	IofOverall                     float64
	IofPercentage                  float64
	InterestRate                   float64
	MinInstallmentAmount           float64
	MaxTotalAmount                 float64
	DisbursementOnlyOnBusinessDays bool
}

func (r *Params) Destroy() {
	FfiDestroyerFloat64{}.Destroy(r.RequestedAmount)
	FfiDestroyerTimestamp{}.Destroy(r.FirstPaymentDate)
	FfiDestroyerTimestamp{}.Destroy(r.RequestedDate)
	FfiDestroyerUint32{}.Destroy(r.Installments)
	FfiDestroyerUint16{}.Destroy(r.DebitServicePercentage)
	FfiDestroyerFloat64{}.Destroy(r.Mdr)
	FfiDestroyerFloat64{}.Destroy(r.TacPercentage)
	FfiDestroyerFloat64{}.Destroy(r.IofOverall)
	FfiDestroyerFloat64{}.Destroy(r.IofPercentage)
	FfiDestroyerFloat64{}.Destroy(r.InterestRate)
	FfiDestroyerFloat64{}.Destroy(r.MinInstallmentAmount)
	FfiDestroyerFloat64{}.Destroy(r.MaxTotalAmount)
	FfiDestroyerBool{}.Destroy(r.DisbursementOnlyOnBusinessDays)
}

type FfiConverterParams struct{}

var FfiConverterParamsINSTANCE = FfiConverterParams{}

func (c FfiConverterParams) Lift(rb RustBufferI) Params {
	return LiftFromRustBuffer[Params](c, rb)
}

func (c FfiConverterParams) Read(reader io.Reader) Params {
	return Params{
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterTimestampINSTANCE.Read(reader),
		FfiConverterTimestampINSTANCE.Read(reader),
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterUint16INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterBoolINSTANCE.Read(reader),
	}
}

func (c FfiConverterParams) Lower(value Params) C.RustBuffer {
	return LowerIntoRustBuffer[Params](c, value)
}

func (c FfiConverterParams) Write(writer io.Writer, value Params) {
	FfiConverterFloat64INSTANCE.Write(writer, value.RequestedAmount)
	FfiConverterTimestampINSTANCE.Write(writer, value.FirstPaymentDate)
	FfiConverterTimestampINSTANCE.Write(writer, value.RequestedDate)
	FfiConverterUint32INSTANCE.Write(writer, value.Installments)
	FfiConverterUint16INSTANCE.Write(writer, value.DebitServicePercentage)
	FfiConverterFloat64INSTANCE.Write(writer, value.Mdr)
	FfiConverterFloat64INSTANCE.Write(writer, value.TacPercentage)
	FfiConverterFloat64INSTANCE.Write(writer, value.IofOverall)
	FfiConverterFloat64INSTANCE.Write(writer, value.IofPercentage)
	FfiConverterFloat64INSTANCE.Write(writer, value.InterestRate)
	FfiConverterFloat64INSTANCE.Write(writer, value.MinInstallmentAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.MaxTotalAmount)
	FfiConverterBoolINSTANCE.Write(writer, value.DisbursementOnlyOnBusinessDays)
}

type FfiDestroyerParams struct{}

func (_ FfiDestroyerParams) Destroy(value Params) {
	value.Destroy()
}

type Response struct {
	Installment                              uint32
	DueDate                                  time.Time
	DisbursementDate                         time.Time
	AccumulatedDays                          int64
	DaysIndex                                float64
	AccumulatedDaysIndex                     float64
	InterestRate                             float64
	InstallmentAmount                        float64
	InstallmentAmountWithoutTac              float64
	TotalAmount                              float64
	DebitService                             float64
	CustomerDebitServiceAmount               float64
	CustomerAmount                           float64
	CalculationBasisForEffectiveInterestRate float64
	MerchantDebitServiceAmount               float64
	MerchantTotalAmount                      float64
	SettledToMerchant                        float64
	MdrAmount                                float64
	EffectiveInterestRate                    float64
	TotalEffectiveCost                       float64
	EirYearly                                float64
	TecYearly                                float64
	EirMonthly                               float64
	TecMonthly                               float64
	TotalIof                                 float64
	ContractAmount                           float64
	ContractAmountWithoutTac                 float64
	TacAmount                                float64
	IofPercentage                            float64
	OverallIof                               float64
	PreDisbursementAmount                    float64
	PaidTotalIof                             float64
	PaidContractAmount                       float64
}

func (r *Response) Destroy() {
	FfiDestroyerUint32{}.Destroy(r.Installment)
	FfiDestroyerTimestamp{}.Destroy(r.DueDate)
	FfiDestroyerTimestamp{}.Destroy(r.DisbursementDate)
	FfiDestroyerInt64{}.Destroy(r.AccumulatedDays)
	FfiDestroyerFloat64{}.Destroy(r.DaysIndex)
	FfiDestroyerFloat64{}.Destroy(r.AccumulatedDaysIndex)
	FfiDestroyerFloat64{}.Destroy(r.InterestRate)
	FfiDestroyerFloat64{}.Destroy(r.InstallmentAmount)
	FfiDestroyerFloat64{}.Destroy(r.InstallmentAmountWithoutTac)
	FfiDestroyerFloat64{}.Destroy(r.TotalAmount)
	FfiDestroyerFloat64{}.Destroy(r.DebitService)
	FfiDestroyerFloat64{}.Destroy(r.CustomerDebitServiceAmount)
	FfiDestroyerFloat64{}.Destroy(r.CustomerAmount)
	FfiDestroyerFloat64{}.Destroy(r.CalculationBasisForEffectiveInterestRate)
	FfiDestroyerFloat64{}.Destroy(r.MerchantDebitServiceAmount)
	FfiDestroyerFloat64{}.Destroy(r.MerchantTotalAmount)
	FfiDestroyerFloat64{}.Destroy(r.SettledToMerchant)
	FfiDestroyerFloat64{}.Destroy(r.MdrAmount)
	FfiDestroyerFloat64{}.Destroy(r.EffectiveInterestRate)
	FfiDestroyerFloat64{}.Destroy(r.TotalEffectiveCost)
	FfiDestroyerFloat64{}.Destroy(r.EirYearly)
	FfiDestroyerFloat64{}.Destroy(r.TecYearly)
	FfiDestroyerFloat64{}.Destroy(r.EirMonthly)
	FfiDestroyerFloat64{}.Destroy(r.TecMonthly)
	FfiDestroyerFloat64{}.Destroy(r.TotalIof)
	FfiDestroyerFloat64{}.Destroy(r.ContractAmount)
	FfiDestroyerFloat64{}.Destroy(r.ContractAmountWithoutTac)
	FfiDestroyerFloat64{}.Destroy(r.TacAmount)
	FfiDestroyerFloat64{}.Destroy(r.IofPercentage)
	FfiDestroyerFloat64{}.Destroy(r.OverallIof)
	FfiDestroyerFloat64{}.Destroy(r.PreDisbursementAmount)
	FfiDestroyerFloat64{}.Destroy(r.PaidTotalIof)
	FfiDestroyerFloat64{}.Destroy(r.PaidContractAmount)
}

type FfiConverterResponse struct{}

var FfiConverterResponseINSTANCE = FfiConverterResponse{}

func (c FfiConverterResponse) Lift(rb RustBufferI) Response {
	return LiftFromRustBuffer[Response](c, rb)
}

func (c FfiConverterResponse) Read(reader io.Reader) Response {
	return Response{
		FfiConverterUint32INSTANCE.Read(reader),
		FfiConverterTimestampINSTANCE.Read(reader),
		FfiConverterTimestampINSTANCE.Read(reader),
		FfiConverterInt64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
		FfiConverterFloat64INSTANCE.Read(reader),
	}
}

func (c FfiConverterResponse) Lower(value Response) C.RustBuffer {
	return LowerIntoRustBuffer[Response](c, value)
}

func (c FfiConverterResponse) Write(writer io.Writer, value Response) {
	FfiConverterUint32INSTANCE.Write(writer, value.Installment)
	FfiConverterTimestampINSTANCE.Write(writer, value.DueDate)
	FfiConverterTimestampINSTANCE.Write(writer, value.DisbursementDate)
	FfiConverterInt64INSTANCE.Write(writer, value.AccumulatedDays)
	FfiConverterFloat64INSTANCE.Write(writer, value.DaysIndex)
	FfiConverterFloat64INSTANCE.Write(writer, value.AccumulatedDaysIndex)
	FfiConverterFloat64INSTANCE.Write(writer, value.InterestRate)
	FfiConverterFloat64INSTANCE.Write(writer, value.InstallmentAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.InstallmentAmountWithoutTac)
	FfiConverterFloat64INSTANCE.Write(writer, value.TotalAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.DebitService)
	FfiConverterFloat64INSTANCE.Write(writer, value.CustomerDebitServiceAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.CustomerAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.CalculationBasisForEffectiveInterestRate)
	FfiConverterFloat64INSTANCE.Write(writer, value.MerchantDebitServiceAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.MerchantTotalAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.SettledToMerchant)
	FfiConverterFloat64INSTANCE.Write(writer, value.MdrAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.EffectiveInterestRate)
	FfiConverterFloat64INSTANCE.Write(writer, value.TotalEffectiveCost)
	FfiConverterFloat64INSTANCE.Write(writer, value.EirYearly)
	FfiConverterFloat64INSTANCE.Write(writer, value.TecYearly)
	FfiConverterFloat64INSTANCE.Write(writer, value.EirMonthly)
	FfiConverterFloat64INSTANCE.Write(writer, value.TecMonthly)
	FfiConverterFloat64INSTANCE.Write(writer, value.TotalIof)
	FfiConverterFloat64INSTANCE.Write(writer, value.ContractAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.ContractAmountWithoutTac)
	FfiConverterFloat64INSTANCE.Write(writer, value.TacAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.IofPercentage)
	FfiConverterFloat64INSTANCE.Write(writer, value.OverallIof)
	FfiConverterFloat64INSTANCE.Write(writer, value.PreDisbursementAmount)
	FfiConverterFloat64INSTANCE.Write(writer, value.PaidTotalIof)
	FfiConverterFloat64INSTANCE.Write(writer, value.PaidContractAmount)
}

type FfiDestroyerResponse struct{}

func (_ FfiDestroyerResponse) Destroy(value Response) {
	value.Destroy()
}

type Error struct {
	err error
}

// Convience method to turn *Error into error
// Avoiding treating nil pointer as non nil error interface
func (err *Error) AsError() error {
	if err == nil {
		return nil
	} else {
		return err
	}
}

func (err Error) Error() string {
	return fmt.Sprintf("Error: %s", err.err.Error())
}

func (err Error) Unwrap() error {
	return err.err
}

// Err* are used for checking error type with `errors.Is`
var ErrErrorInvalidParams = fmt.Errorf("ErrorInvalidParams")
var ErrErrorCalculationError = fmt.Errorf("ErrorCalculationError")

// Variant structs
type ErrorInvalidParams struct {
}

func NewErrorInvalidParams() *Error {
	return &Error{err: &ErrorInvalidParams{}}
}

func (e ErrorInvalidParams) destroy() {
}

func (err ErrorInvalidParams) Error() string {
	return fmt.Sprint("InvalidParams")
}

func (self ErrorInvalidParams) Is(target error) bool {
	return target == ErrErrorInvalidParams
}

type ErrorCalculationError struct {
}

func NewErrorCalculationError() *Error {
	return &Error{err: &ErrorCalculationError{}}
}

func (e ErrorCalculationError) destroy() {
}

func (err ErrorCalculationError) Error() string {
	return fmt.Sprint("CalculationError")
}

func (self ErrorCalculationError) Is(target error) bool {
	return target == ErrErrorCalculationError
}

type FfiConverterError struct{}

var FfiConverterErrorINSTANCE = FfiConverterError{}

func (c FfiConverterError) Lift(eb RustBufferI) *Error {
	return LiftFromRustBuffer[*Error](c, eb)
}

func (c FfiConverterError) Lower(value *Error) C.RustBuffer {
	return LowerIntoRustBuffer[*Error](c, value)
}

func (c FfiConverterError) Read(reader io.Reader) *Error {
	errorID := readUint32(reader)

	switch errorID {
	case 1:
		return &Error{&ErrorInvalidParams{}}
	case 2:
		return &Error{&ErrorCalculationError{}}
	default:
		panic(fmt.Sprintf("Unknown error code %d in FfiConverterError.Read()", errorID))
	}
}

func (c FfiConverterError) Write(writer io.Writer, value *Error) {
	switch variantValue := value.err.(type) {
	case *ErrorInvalidParams:
		writeInt32(writer, 1)
	case *ErrorCalculationError:
		writeInt32(writer, 2)
	default:
		_ = variantValue
		panic(fmt.Sprintf("invalid error value `%v` in FfiConverterError.Write", value))
	}
}

type FfiDestroyerError struct{}

func (_ FfiDestroyerError) Destroy(value *Error) {
	switch variantValue := value.err.(type) {
	case ErrorInvalidParams:
		variantValue.destroy()
	case ErrorCalculationError:
		variantValue.destroy()
	default:
		_ = variantValue
		panic(fmt.Sprintf("invalid error value `%v` in FfiDestroyerError.Destroy", value))
	}
}

type FfiConverterSequenceDownPaymentResponse struct{}

var FfiConverterSequenceDownPaymentResponseINSTANCE = FfiConverterSequenceDownPaymentResponse{}

func (c FfiConverterSequenceDownPaymentResponse) Lift(rb RustBufferI) []DownPaymentResponse {
	return LiftFromRustBuffer[[]DownPaymentResponse](c, rb)
}

func (c FfiConverterSequenceDownPaymentResponse) Read(reader io.Reader) []DownPaymentResponse {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]DownPaymentResponse, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterDownPaymentResponseINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceDownPaymentResponse) Lower(value []DownPaymentResponse) C.RustBuffer {
	return LowerIntoRustBuffer[[]DownPaymentResponse](c, value)
}

func (c FfiConverterSequenceDownPaymentResponse) Write(writer io.Writer, value []DownPaymentResponse) {
	if len(value) > math.MaxInt32 {
		panic("[]DownPaymentResponse is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterDownPaymentResponseINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceDownPaymentResponse struct{}

func (FfiDestroyerSequenceDownPaymentResponse) Destroy(sequence []DownPaymentResponse) {
	for _, value := range sequence {
		FfiDestroyerDownPaymentResponse{}.Destroy(value)
	}
}

type FfiConverterSequenceResponse struct{}

var FfiConverterSequenceResponseINSTANCE = FfiConverterSequenceResponse{}

func (c FfiConverterSequenceResponse) Lift(rb RustBufferI) []Response {
	return LiftFromRustBuffer[[]Response](c, rb)
}

func (c FfiConverterSequenceResponse) Read(reader io.Reader) []Response {
	length := readInt32(reader)
	if length == 0 {
		return nil
	}
	result := make([]Response, 0, length)
	for i := int32(0); i < length; i++ {
		result = append(result, FfiConverterResponseINSTANCE.Read(reader))
	}
	return result
}

func (c FfiConverterSequenceResponse) Lower(value []Response) C.RustBuffer {
	return LowerIntoRustBuffer[[]Response](c, value)
}

func (c FfiConverterSequenceResponse) Write(writer io.Writer, value []Response) {
	if len(value) > math.MaxInt32 {
		panic("[]Response is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	for _, item := range value {
		FfiConverterResponseINSTANCE.Write(writer, item)
	}
}

type FfiDestroyerSequenceResponse struct{}

func (FfiDestroyerSequenceResponse) Destroy(sequence []Response) {
	for _, value := range sequence {
		FfiDestroyerResponse{}.Destroy(value)
	}
}

func CalculateDownPaymentPlan(params DownPaymentParams) ([]DownPaymentResponse, *Error) {
	_uniffiRV, _uniffiErr := rustCallWithError[Error](FfiConverterError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_payment_plan_uniffi_fn_func_calculate_down_payment_plan(FfiConverterDownPaymentParamsINSTANCE.Lower(params), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []DownPaymentResponse
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceDownPaymentResponseINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}

func CalculatePaymentPlan(params Params) ([]Response, *Error) {
	_uniffiRV, _uniffiErr := rustCallWithError[Error](FfiConverterError{}, func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer{
			inner: C.uniffi_payment_plan_uniffi_fn_func_calculate_payment_plan(FfiConverterParamsINSTANCE.Lower(params), _uniffiStatus),
		}
	})
	if _uniffiErr != nil {
		var _uniffiDefaultValue []Response
		return _uniffiDefaultValue, _uniffiErr
	} else {
		return FfiConverterSequenceResponseINSTANCE.Lift(_uniffiRV), _uniffiErr
	}
}
