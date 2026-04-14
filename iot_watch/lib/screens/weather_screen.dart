import 'package:flutter/material.dart';
import 'package:iot_watch/domain/sensor_repository.dart';

class WeatherAppScreen extends StatelessWidget {
  final SensorRepository repository;

  const WeatherAppScreen({super.key, required this.repository});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF333333),
      body: StreamBuilder<WatchUiState>(
        stream: repository.watchStateStream,
        builder: (context, snapshot) {
          final temp = snapshot.data?.temperature ?? '--';
          final pressure = snapshot.data?.barometricPressure ?? '--';

          return GestureDetector(
            onTap: () => Navigator.of(context).pop(),
            child: Padding(
              padding: const EdgeInsets.all(12.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Padding(
                    padding: EdgeInsets.only(left: 4.0, bottom: 4.0),
                    child: Text('Weather', style: TextStyle(color: Colors.white70, fontSize: 14)),
                  ),
                  Expanded(
                    child: Container(
                      width: double.infinity,
                      decoration: BoxDecoration(
                        color: const Color(0xFFF2ECEC),
                        borderRadius: BorderRadius.circular(20),
                      ),
                      padding: const EdgeInsets.symmetric(horizontal: 8.0, vertical: 8.0),
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                        children: [
                          Column(
                            children: [
                              Row(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                  const SizedBox(
                                    width: 44,
                                    height: 36,
                                    child: Stack(
                                      children: [
                                        Positioned(top: 0, left: 0, child: Icon(Icons.wb_sunny, color: Colors.yellow, size: 26)),
                                        Positioned(bottom: 0, right: 0, child: Icon(Icons.cloud_outlined, color: Colors.lightBlue, size: 28)),
                                      ],
                                    ),
                                  ),
                                  const SizedBox(width: 8),
                                  Text('$temp°F', style: const TextStyle(fontSize: 32, fontWeight: FontWeight.w500, color: Colors.black)),
                                ],
                              ),
                              const SizedBox(height: 2),
                              const Text('Live Sensor Data', style: TextStyle(fontSize: 10, color: Colors.black54)),
                            ],
                          ),
                          const Divider(color: Colors.grey, thickness: 1, height: 12),
                          Column(
                            children: [
                              const FittedBox(
                                fit: BoxFit.scaleDown,
                                child: Text('Barometric Pressure', style: TextStyle(fontSize: 10, color: Colors.black87)),
                              ),
                              const SizedBox(height: 2),
                              FittedBox(
                                fit: BoxFit.scaleDown,
                                child: RichText(
                                  text: TextSpan(children: [
                                    TextSpan(text: pressure, style: const TextStyle(fontSize: 24, color: Colors.black)),
                                    const TextSpan(text: ' mmHG', style: TextStyle(fontSize: 12, color: Colors.black)),
                                  ]),
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