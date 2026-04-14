import 'package:flutter/material.dart';
import 'package:iot_watch/domain/sensor_repository.dart';

class HealthAppScreen extends StatelessWidget {
  final SensorRepository repository;

  const HealthAppScreen({super.key, required this.repository});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF333333),
      body: StreamBuilder<WatchUiState>(
        stream: repository.watchStateStream,
        builder: (context, snapshot) {
          // Extract data safely, providing fallbacks if the stream hasn't emitted yet
          final state = snapshot.data;
          final heartRate = state?.heartRate ?? '--';
          final bloodPressure = state?.bloodPressure ?? '--/--';
          final steps = state?.steps ?? '--';
          final distance = state?.distance ?? '--';

          return GestureDetector(
            onTap: () => Navigator.of(context).pop(),
            child: Padding(
              padding: const EdgeInsets.all(12.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Padding(
                    padding: EdgeInsets.only(left: 4.0, bottom: 4.0),
                    child: Text('Health', style: TextStyle(color: Colors.white70, fontSize: 14)),
                  ),
                  Expanded(
                    child: Container(
                      decoration: BoxDecoration(
                        color: const Color(0xFFF2ECEC),
                        borderRadius: BorderRadius.circular(20),
                      ),
                      padding: const EdgeInsets.all(8.0),
                      child: Column(
                        children: [
                          Expanded(
                            flex: 5,
                            child: Row(
                              children: [
                                Expanded(
                                  flex: 11,
                                  child: Container(
                                    padding: const EdgeInsets.all(4.0),
                                    decoration: BoxDecoration(
                                      color: Colors.white,
                                      borderRadius: BorderRadius.circular(16),
                                    ),
                                    child: FittedBox(
                                      fit: BoxFit.scaleDown,
                                      alignment: Alignment.centerLeft,
                                      child: Column(
                                        crossAxisAlignment: CrossAxisAlignment.start,
                                        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                                        children: [
                                          const Row(
                                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                                            children: [
                                              Text('Heart Rate', style: TextStyle(fontSize: 10, color: Colors.black87)),
                                              SizedBox(width: 4),
                                              Icon(Icons.favorite_border, color: Colors.red, size: 14),
                                            ],
                                          ),
                                          RichText(
                                            text: TextSpan(children: [
                                              TextSpan(text: heartRate, style: const TextStyle(fontSize: 22, fontWeight: FontWeight.w500, color: Colors.black)),
                                              const TextSpan(text: ' BPM', style: TextStyle(fontSize: 9, color: Colors.black54)),
                                            ]),
                                          ),
                                          const SizedBox(height: 2),
                                          const Text('Blood Pressure', style: TextStyle(fontSize: 10, color: Colors.black87)),
                                          Text(bloodPressure, style: const TextStyle(fontSize: 18, color: Colors.black)),
                                        ],
                                      ),
                                    ),
                                  ),
                                ),
                                const SizedBox(width: 8),
                                const Expanded(
                                  flex: 9,
                                  child: Column(
                                    mainAxisAlignment: MainAxisAlignment.center,
                                    children: [
                                      Text('Today', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, color: Colors.black)),
                                      SizedBox(height: 6),
                                      Text('Live Sync', style: TextStyle(fontSize: 11, color: Colors.black87)),
                                    ],
                                  ),
                                ),
                              ],
                            ),
                          ),
                          const SizedBox(height: 8),
                          Expanded(
                            flex: 3,
                            child: Row(
                              children: [
                                Expanded(
                                  child: Padding(
                                    padding: const EdgeInsets.only(left: 4.0),
                                    child: Column(
                                      crossAxisAlignment: CrossAxisAlignment.start,
                                      mainAxisAlignment: MainAxisAlignment.center,
                                      children: [
                                        const Text('Daily Steps', style: TextStyle(fontSize: 10, color: Colors.black87)),
                                        FittedBox(
                                          fit: BoxFit.scaleDown,
                                          alignment: Alignment.centerLeft,
                                          child: RichText(
                                            text: TextSpan(children: [
                                              TextSpan(text: steps, style: const TextStyle(fontSize: 16, color: Colors.black)),
                                              const TextSpan(text: ' steps', style: TextStyle(fontSize: 9, color: Colors.black54)),
                                            ]),
                                          ),
                                        ),
                                      ],
                                    ),
                                  ),
                                ),
                                Expanded(
                                  child: Column(
                                    crossAxisAlignment: CrossAxisAlignment.start,
                                    mainAxisAlignment: MainAxisAlignment.center,
                                    children: [
                                      const Text('Distance', style: TextStyle(fontSize: 10, color: Colors.black87)),
                                      FittedBox(
                                        fit: BoxFit.scaleDown,
                                        alignment: Alignment.centerLeft,
                                        child: RichText(
                                          text: TextSpan(children: [
                                            TextSpan(text: distance, style: const TextStyle(fontSize: 16, color: Colors.black)),
                                            const TextSpan(text: ' ft', style: TextStyle(fontSize: 9, color: Colors.black54)),
                                          ]),
                                        ),
                                      ),
                                    ],
                                  ),
                                ),
                              ],
                            ),
                          )
                        ],
                      ),
                    ),
                  ),
                ],
              ),
            ),
          );
        },
      ),
    );
  }
}