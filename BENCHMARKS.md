# Serialization benchmarks

### `postcard` vs custom
| Benchmark Name                                   | Avg Time [ns] | Avg % Improvement (compared to `postcard`) |
|--------------------------------------------------|:-------------:|:------------------------------------------:|
| Message(Request::Ping)::serialize                |   10.393      |                55.50%                     |
| Message(Request::Handshake)::serialize           |   22.824      |                32.92%                     |
| Message(Request::PostResults)::serialize         |   18.207      |                42.01%                     |
| Message(Request::PostStats)::serialize           |   31.204      |                62.09%                     |
| Message(Request::SendNotification)::serialize    |   28.818      |                19.28%                     |
| Message(Request::GetSettings)::serialize         |   10.107      |                54.98%                     |
| Message(Request::UpdateCheck)::serialize         |   19.148      |                30.45%                     |
| Message(Request::NextUpdateChunk)::serialize     |   12.788      |                48.80%                     |
| Message(Request::ReportFirmwareUpdate)::serialize|   11.523      |                51.44%                     |
| Message(Request::Bye)::serialize                 |   10.556      |                53.40%                     |