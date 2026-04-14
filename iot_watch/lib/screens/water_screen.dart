import 'package:flutter/material.dart';
import 'package:iot_watch/domain/sensor_repository.dart';
import 'package:iot_watch/src/rust/api.dart';

class WaterAppScreen extends StatelessWidget {
  final SensorRepository repository;

  const WaterAppScreen({super.key, required this.repository});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF333333),
      body: StreamBuilder<WatchUiState>(
        stream: repository.watchStateStream,
        builder: (context, snapshot) {
          final isWaterDetected = snapshot.data?.waterInDevice ?? false;

          final detectionText = isWaterDetected
              ? 'Water Detected!'
              : 'No Current Water Detection';
          final statusText = isWaterDetected
              ? 'Ejecting...'
              : 'Monitoring Device';

          return GestureDetector(
            onTap: () => Navigator.of(context).pop(),
            child: Padding(
              padding: const EdgeInsets.all(12.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Padding(
                    padding: EdgeInsets.only(left: 4.0, bottom: 4.0),
                    child: Text(
                      'Water Removal',
                      style: TextStyle(color: Colors.white70, fontSize: 14),
                    ),
                  ),
                  Expanded(
                    child: Container(
                      width: double.infinity,
                      decoration: BoxDecoration(
                        color: const Color(0xFFF2ECEC),
                        borderRadius: BorderRadius.circular(20),
                      ),
                      padding: const EdgeInsets.symmetric(
                        horizontal: 8.0,
                        vertical: 12.0,
                      ),
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          FittedBox(
                            fit: BoxFit.scaleDown,
                            child: Text(
                              detectionText,
                              style: const TextStyle(
                                fontSize: 11,
                                color: Colors.black87,
                              ),
                            ),
                          ),
                          Container(
                            width: 44,
                            height: 44,
                            decoration: BoxDecoration(
                              color: isWaterDetected
                                  ? Colors.blueAccent
                                  : const Color(0xFFC43E3E),
                              borderRadius: BorderRadius.circular(10),
                            ),
                            child: Stack(
                              alignment: Alignment.center,
                              children: [
                                const Icon(
                                  Icons.water_drop_outlined,
                                  color: Colors.white,
                                  size: 30,
                                ),
                                if (!isWaterDetected)
                                  const Positioned(
                                    bottom: 8,
                                    child: Icon(
                                      Icons.close,
                                      color: Colors.white,
                                      size: 14,
                                    ),
                                  ),
                              ],
                            ),
                          ),
                          Column(
                            children: [
                              const FittedBox(
                                fit: BoxFit.scaleDown,
                                child: Text(
                                  'Water Removal Feature Status:',
                                  style: TextStyle(
                                    fontSize: 10,
                                    color: Colors.black87,
                                  ),
                                ),
                              ),
                              const SizedBox(height: 2),
                              FittedBox(
                                fit: BoxFit.scaleDown,
                                child: Text(
                                  statusText,
                                  style: const TextStyle(
                                    fontSize: 10,
                                    color: Colors.black87,
                                    fontStyle: FontStyle.italic,
                                  ),
                                ),
                              ),
                            ],
                          ),
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
