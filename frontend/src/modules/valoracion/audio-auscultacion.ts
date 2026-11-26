export class AudioAuscultacion {
  private contexto?: AudioContext;
  private fuente?: AudioBufferSourceNode;
  private ganancia?: GainNode;
  private solicitud = 0;

  async alternar(cardiaco: boolean): Promise<boolean> {
    if (this.fuente) {
      this.detener();
      return false;
    }
    const solicitud = ++this.solicitud;
    const contexto = this.contexto ??= new AudioContext();
    await contexto.resume();
    if (solicitud !== this.solicitud) return false;

    const frecuencia = contexto.sampleRate;
    const duracion = cardiaco ? 0.82 : 2.4;
    const bufer = contexto.createBuffer(1, Math.floor(duracion * frecuencia), frecuencia);
    const muestras = bufer.getChannelData(0);
    let filtrado = 0;
    for (let indice = 0; indice < muestras.length; indice++) {
      const t = indice / frecuencia;
      if (cardiaco) {
        const a = Math.max(0, t - 0.08);
        const b = Math.max(0, t - 0.28);
        muestras[indice] = 0.35 * Math.sin(a * 340) * Math.exp(-45 * a)
          + 0.26 * Math.sin(b * 410) * Math.exp(-55 * b);
      } else {
        filtrado = 0.84 * filtrado + 0.16 * (Math.random() * 2 - 1);
        const envolvente = Math.pow(Math.sin(Math.PI * t / duracion), 2);
        muestras[indice] = filtrado * envolvente * 0.68;
      }
    }
    this.fuente = contexto.createBufferSource();
    this.ganancia = contexto.createGain();
    this.ganancia.gain.value = 0.24;
    this.fuente.buffer = bufer;
    this.fuente.loop = true;
    this.fuente.connect(this.ganancia).connect(contexto.destination);
    this.fuente.start();
    return true;
  }

  detener(): void {
    this.solicitud++;
    this.fuente?.stop();
    this.fuente?.disconnect();
    this.ganancia?.disconnect();
    this.fuente = undefined;
    this.ganancia = undefined;
    if (this.contexto) void this.contexto.close().catch(() => {});
    this.contexto = undefined;
  }
}
