package evidence_api

type CC_Type int32

const (
	TYPE_CC_NONE CC_Type = -1
	TYPE_CC_TPM  CC_Type = 0
	TYPE_CC_TDX  CC_Type = 1
	TYPE_CC_SEV  CC_Type = 2
	TYPE_CC_CCA  CC_Type = 3
)

func (t CC_Type) String() string {
	switch t {
	case TYPE_CC_NONE:
		return "NONE"
	case TYPE_CC_TPM:
		return "TPM"
	case TYPE_CC_TDX:
		return "TDX"
	case TYPE_CC_SEV:
		return "SEV"
	case TYPE_CC_CCA:
		return "CCA"
	}
	return ""
}

type EvidenceAPI interface {
	GetDefaultAlgorithm() (TCG_ALG, error)

	GetCCReport(nonce, userData []byte, extraArgs map[string]any) (Report, error)
	DumpCCReport(reportBytes []byte) error

	GetMeasurementCount() (int, error)
	GetCCMeasurement(index int, alg TCG_ALG) (TcgDigest, error)

	// two optional params can be specified for GetCCEventLog()
	// which are start and count
	GetCCEventLog(params ...int32) ([]FormatedTcgEvent, error)
	ReplayCCEventLog(formatedEventLogs []FormatedTcgEvent) map[int]map[TCG_ALG][]byte
}
