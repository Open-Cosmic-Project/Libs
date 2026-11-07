#ifndef COSMIC_USB_H
#define COSMIC_USB_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Representa un dispositivo USB detectado por el subsistema.
 */
struct cosmic_usb_device {
    uint16_t vendor_id;
    uint16_t product_id;
    char serial[256];
    int fd;
    int connected;
};

/**
 * Inicializa el subsistema USB.
 * Retorna 0 en éxito, -1 en error.
 */
int cosmic_usb_init(void);

/**
 * Apaga el subsistema USB y libera recursos.
 */
void cosmic_usb_shutdown(void);

/**
 * Escanea dispositivos USB conectados.
 * @param devices    Arreglo de salida para dispositivos encontrados.
 * @param max_devices Capacidad máxima del arreglo.
 * @return Número de dispositivos encontrados, o -1 en error.
 */
int cosmic_usb_scan(struct cosmic_usb_device *devices, int max_devices);

/**
 * Abre un dispositivo USB para comunicación.
 * @param dev Dispositivo a abrir.
 * @return 0 en éxito, -1 en error.
 */
int cosmic_usb_open(struct cosmic_usb_device *dev);

/**
 * Cierra un dispositivo USB abierto.
 * @param dev Dispositivo a cerrar.
 */
void cosmic_usb_close(struct cosmic_usb_device *dev);

/**
 * Lee datos de un dispositivo USB.
 * @param dev        Dispositivo origen.
 * @param buf        Buffer de destino.
 * @param len        Cantidad máxima de bytes a leer.
 * @param timeout_ms Tiempo límite en milisegundos.
 * @return Bytes leídos, o -1 en error.
 */
int cosmic_usb_read(struct cosmic_usb_device *dev, void *buf, int len,
                    int timeout_ms);

/**
 * Escribe datos a un dispositivo USB.
 * @param dev        Dispositivo destino.
 * @param buf        Buffer de origen.
 * @param len        Cantidad de bytes a escribir.
 * @param timeout_ms Tiempo límite en milisegundos.
 * @return Bytes escritos, o -1 en error.
 */
int cosmic_usb_write(struct cosmic_usb_device *dev, const void *buf, int len,
                     int timeout_ms);

#ifdef __cplusplus
}
#endif

#endif /* COSMIC_USB_H */
